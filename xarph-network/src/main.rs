//! Xarph Network Monitor
//!
//! Reads real network interface data from NetworkManager via D-Bus.

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, CssProvider, Label, Orientation, ProgressBar, gdk,
    glib,
};
use std::collections::HashMap;

const APP_CSS: &str = r#"
window { background-color: rgba(18, 18, 26, 0.98); }
.network-monitor { padding: 24px; }
.network-title { font-size: 20px; font-weight: 700; color: rgba(230, 230, 240, 0.95); margin-bottom: 16px; }
.network-interface {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 10px;
    padding: 14px 16px;
    margin-bottom: 8px;
}
.interface-name { font-weight: 600; color: rgba(220, 220, 235, 0.92); min-width: 80px; }
.interface-status { color: rgba(120, 200, 160, 0.9); font-size: 16px; margin: 0 8px; }
.interface-type { color: rgba(180, 180, 200, 0.6); font-size: 12px; margin: 0 8px; }
.interface-ips { color: rgba(200, 200, 215, 0.8); font-size: 12px; margin: 0 8px; }
.interface-speed { color: rgba(120, 180, 255, 0.85); font-size: 12px; margin: 0 8px; }
progressbar trough { background: rgba(255, 255, 255, 0.06); border-radius: 4px; min-height: 8px; }
progressbar progress { background: rgba(120, 180, 255, 0.7); border-radius: 4px; }
"#;

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub interface_type: String,
    pub is_up: bool,
    pub ip_addresses: Vec<String>,
    pub speed: Option<u64>,
    pub bytes_received: u64,
    pub bytes_sent: u64,
}

fn fetch_interfaces_sync() -> Vec<NetworkInterface> {
    let output = std::process::Command::new("nmcli")
        .args([
            "-t",
            "-f",
            "NAME,TYPE,STATE,DEVICE",
            "connection",
            "show",
            "--active",
        ])
        .output();

    let mut interfaces = Vec::new();
    let mut seen_devices = HashMap::new();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 4 {
                continue;
            }
            let conn_type = parts[1];
            let state = parts[2];
            let device = parts[3];

            if seen_devices.contains_key(device) {
                continue;
            }
            seen_devices.insert(device.to_string(), true);

            let iface_type = match conn_type {
                "802-3-ethernet" => "Ethernet",
                "802-11-wireless" => "WiFi",
                "loopback" => "Loopback",
                _ => "Other",
            };

            interfaces.push(NetworkInterface {
                name: device.to_string(),
                interface_type: iface_type.to_string(),
                is_up: state == "activated",
                ip_addresses: Vec::new(),
                speed: None,
                bytes_received: 0,
                bytes_sent: 0,
            });
        }
    }

    if let Ok(out) = std::process::Command::new("ip")
        .args(["-o", "-f", "inet", "addr", "show"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let dev = parts[1];
                let addr = parts[3];
                if let Some(iface) = interfaces.iter_mut().find(|i| i.name == dev) {
                    iface.ip_addresses.push(addr.to_string());
                }
            }
        }
    }

    for iface in &mut interfaces {
        let rx_path = format!("/sys/class/net/{}/statistics/rx_bytes", iface.name);
        let tx_path = format!("/sys/class/net/{}/statistics/tx_bytes", iface.name);
        if let Ok(rx) = std::fs::read_to_string(&rx_path) {
            iface.bytes_received = rx.trim().parse().unwrap_or(0);
        }
        if let Ok(tx) = std::fs::read_to_string(&tx_path) {
            iface.bytes_sent = tx.trim().parse().unwrap_or(0);
        }

        let speed_path = format!("/sys/class/net/{}/speed", iface.name);
        if let Ok(speed) = std::fs::read_to_string(&speed_path) {
            let s: i64 = speed.trim().parse().unwrap_or(-1);
            if s > 0 {
                iface.speed = Some(s as u64);
            }
        }
    }

    interfaces
}

fn build_interface_widget(iface: &NetworkInterface) -> Box {
    let interface_box = Box::new(Orientation::Horizontal, 12);
    interface_box.add_css_class("network-interface");

    let name_label = Label::new(Some(&iface.name));
    name_label.add_css_class("interface-name");
    interface_box.append(&name_label);

    let status_label = Label::new(Some(if iface.is_up {
        "\u{2713}"
    } else {
        "\u{2717}"
    }));
    status_label.add_css_class("interface-status");
    interface_box.append(&status_label);

    let type_label = Label::new(Some(&iface.interface_type));
    type_label.add_css_class("interface-type");
    interface_box.append(&type_label);

    if !iface.ip_addresses.is_empty() {
        let ip_label = Label::new(Some(&iface.ip_addresses.join(", ")));
        ip_label.add_css_class("interface-ips");
        interface_box.append(&ip_label);
    }

    if let Some(speed) = iface.speed {
        let speed_label = Label::new(Some(&format!("{speed} Mbps")));
        speed_label.add_css_class("interface-speed");
        interface_box.append(&speed_label);
    }

    let rx_mb = iface.bytes_received as f64 / (1024.0 * 1024.0);
    let tx_mb = iface.bytes_sent as f64 / (1024.0 * 1024.0);

    let rx_bar = ProgressBar::new();
    rx_bar.set_fraction((rx_mb / 1000.0).min(1.0));
    rx_bar.set_show_text(true);
    rx_bar.set_text(Some(&format!("\u{2193} {rx_mb:.1} MB")));
    interface_box.append(&rx_bar);

    let tx_bar = ProgressBar::new();
    tx_bar.set_fraction((tx_mb / 1000.0).min(1.0));
    tx_bar.set_show_text(true);
    tx_bar.set_text(Some(&format!("\u{2191} {tx_mb:.1} MB")));
    interface_box.append(&tx_bar);

    interface_box
}

fn main() {
    let application = Application::builder()
        .application_id("com.xarph.network")
        .build();

    application.connect_activate(|app| {
        let provider = CssProvider::new();
        provider.load_from_data(APP_CSS);
        if let Some(display) = gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let container = Box::new(Orientation::Vertical, 8);
        container.add_css_class("network-monitor");

        let title_label = Label::new(Some("Network"));
        title_label.add_css_class("network-title");
        container.append(&title_label);

        // Content box that will be refreshed
        let content_box = Box::new(Orientation::Vertical, 0);
        container.append(&content_box);

        // Initial populate
        let interfaces = fetch_interfaces_sync();
        if interfaces.is_empty() {
            let empty_label = Label::new(Some("No active network connections"));
            empty_label.add_css_class("network-title");
            content_box.append(&empty_label);
        } else {
            for iface in &interfaces {
                content_box.append(&build_interface_widget(iface));
            }
        }

        // Periodic refresh every 5 seconds
        let content_clone = content_box.clone();
        glib::timeout_add_seconds_local(5, move || {
            // Clear old content
            while let Some(child) = content_clone.first_child() {
                content_clone.remove(&child);
            }

            let interfaces = fetch_interfaces_sync();
            if interfaces.is_empty() {
                let empty_label = Label::new(Some("No active network connections"));
                empty_label.add_css_class("network-title");
                content_clone.append(&empty_label);
            } else {
                for iface in &interfaces {
                    content_clone.append(&build_interface_widget(&iface));
                }
            }

            glib::ControlFlow::Continue
        });

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Xarph Network Monitor")
            .default_width(700)
            .default_height(450)
            .child(&container)
            .build();

        window.present();
    });

    application.run();
}
