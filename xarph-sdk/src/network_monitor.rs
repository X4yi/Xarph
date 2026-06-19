use std::collections::HashMap;
use std::process::Command;

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

pub struct NetworkMonitor;

impl NetworkMonitor {
    pub fn fetch_interfaces() -> Vec<NetworkInterface> {
        let mut interfaces = Vec::new();
        let mut seen_devices = HashMap::new();

        // Get active connections from nmcli
        if let Ok(out) = Command::new("nmcli")
            .args([
                "-t",
                "-f",
                "NAME,TYPE,STATE,DEVICE",
                "connection",
                "show",
                "--active",
            ])
            .output()
        {
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

        // Get IP addresses
        if let Ok(out) = Command::new("ip")
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

        // Get stats from sysfs
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
}
