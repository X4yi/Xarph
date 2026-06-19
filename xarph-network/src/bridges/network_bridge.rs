/// Network bridge: exposes network monitoring to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, interface_name)]
        #[qproperty(QString, ip_address)]
        #[qproperty(u64, rx_bytes)]
        #[qproperty(u64, tx_bytes)]
        #[qproperty(bool, is_connected)]
        #[qproperty(i32, interface_count)]
        #[namespace = "xarph"]
        type NetworkBridge = super::NetworkBridgeRust;

        #[qinvokable]
        fn load_interfaces(self: Pin<&mut Self>) -> QString;

        #[qinvokable]
        fn get_network_stats(&self) -> QString;

        #[qinvokable]
        fn is_online(&self) -> bool;
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct NetworkBridgeRust {
    interface_name: QString,
    ip_address: QString,
    rx_bytes: u64,
    tx_bytes: u64,
    is_connected: bool,
    interface_count: i32,
}

impl qobject::NetworkBridge {
    pub fn load_interfaces(mut self: Pin<&mut Self>) -> QString {
        let interfaces = xarph_sdk::network_monitor::NetworkMonitor::fetch_interfaces();
        let count = interfaces.len() as i32;
        self.as_mut().set_interface_count(count);

        if let Some(iface) = interfaces.first() {
            self.as_mut().set_interface_name(QString::from(&iface.name));
            self.as_mut()
                .set_ip_address(QString::from(&iface.ip_addresses.join(", ")));
            self.as_mut().set_rx_bytes(iface.bytes_received);
            self.as_mut().set_tx_bytes(iface.bytes_sent);
            self.as_mut().set_is_connected(iface.is_up);
        }

        let lines: Vec<String> = interfaces
            .iter()
            .map(|i| {
                format!(
                    "{}|{}|{}|{}|{}",
                    i.name,
                    i.interface_type,
                    i.ip_addresses.join(", "),
                    i.is_up,
                    i.bytes_received + i.bytes_sent
                )
            })
            .collect();
        QString::from(&lines.join("\n"))
    }

    pub fn get_network_stats(&self) -> QString {
        let interfaces = xarph_sdk::network_monitor::NetworkMonitor::fetch_interfaces();
        if let Some(iface) = interfaces.first() {
            let rx_mb = iface.bytes_received as f64 / 1048576.0;
            let tx_mb = iface.bytes_sent as f64 / 1048576.0;
            QString::from(&format!(
                "{}: RX {:.1} MB | TX {:.1} MB",
                iface.name, rx_mb, tx_mb
            ))
        } else {
            QString::from("No network interfaces found")
        }
    }

    pub fn is_online(&self) -> bool {
        *self.is_connected()
    }
}
