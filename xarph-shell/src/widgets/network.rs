use super::ShellWidget;
use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation, Widget, glib};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub struct NetworkWidget {
    /// Network interface to monitor. If None, auto-detects the first active one.
    pub interface: Option<String>,
    /// Refresh interval in milliseconds.
    pub interval_ms: u64,
}

impl Default for NetworkWidget {
    fn default() -> Self {
        Self {
            interface: None,
            interval_ms: 2000,
        }
    }
}

#[derive(Clone, Debug)]
struct NetStats {
    iface: String,
    state: String,
    rx_kbps: f64,
    tx_kbps: f64,
}

fn detect_interface() -> Option<String> {
    let net_dir = PathBuf::from("/sys/class/net");
    if let Ok(entries) = fs::read_dir(&net_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == "lo" {
                continue;
            }
            let state_path = net_dir.join(&name).join("operstate");
            if let Ok(state) = fs::read_to_string(&state_path) {
                if state.trim() == "up" {
                    return Some(name);
                }
            }
        }
    }
    None
}

fn read_rx_tx(iface: &str) -> (u64, u64) {
    let rx_path = format!("/sys/class/net/{}/statistics/rx_bytes", iface);
    let tx_path = format!("/sys/class/net/{}/statistics/tx_bytes", iface);
    let rx = fs::read_to_string(&rx_path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0u64);
    let tx = fs::read_to_string(&tx_path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0u64);
    (rx, tx)
}

impl ShellWidget for NetworkWidget {
    fn build(&self) -> Widget {
        let container = Box::new(Orientation::Horizontal, 6);
        container.add_css_class("network-widget");

        let icon_label = Label::new(Some("󰈀"));
        let info_label = Label::builder()
            .css_classes(vec!["network-label".to_string()])
            .label("...")
            .build();

        container.append(&icon_label);
        container.append(&info_label);

        let iface_override = self.interface.clone();
        let interval = self.interval_ms;
        let (tx, rx) = mpsc::channel::<NetStats>();

        thread::spawn(move || {
            let iface = iface_override
                .or_else(detect_interface)
                .unwrap_or_else(|| "lo".to_string());

            let (mut prev_rx, mut prev_tx) = read_rx_tx(&iface);
            loop {
                thread::sleep(std::time::Duration::from_millis(interval));

                let state = fs::read_to_string(format!("/sys/class/net/{}/operstate", iface))
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                let (cur_rx, cur_tx) = read_rx_tx(&iface);
                let rx_kbps =
                    (cur_rx.saturating_sub(prev_rx)) as f64 / 1024.0 / (interval as f64 / 1000.0);
                let tx_kbps =
                    (cur_tx.saturating_sub(prev_tx)) as f64 / 1024.0 / (interval as f64 / 1000.0);
                prev_rx = cur_rx;
                prev_tx = cur_tx;

                let _ = tx.send(NetStats {
                    iface: iface.clone(),
                    state,
                    rx_kbps,
                    tx_kbps,
                });
            }
        });

        glib::timeout_add_local(std::time::Duration::from_millis(interval), move || {
            while let Ok(s) = rx.try_recv() {
                // Choose icon based on connection state and interface name
                let icon = if s.state == "up" {
                    if s.iface.starts_with('w') {
                        "󰤨"
                    } else {
                        "󰈀"
                    }
                } else {
                    "󰤭"
                };
                icon_label.set_text(icon);
                info_label.set_text(&format!("↓{:.0}↑{:.0} KB/s", s.rx_kbps, s.tx_kbps));
            }
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}
