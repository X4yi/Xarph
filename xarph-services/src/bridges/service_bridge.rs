/// Service bridge: exposes systemd services to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, service_name)]
        #[qproperty(QString, status)]
        #[qproperty(bool, active)]
        #[qproperty(bool, enabled)]
        #[qproperty(i32, service_count)]
        #[namespace = "xarph"]
        type ServiceBridge = super::ServiceBridgeRust;

        #[qinvokable]
        fn load_services(self: Pin<&mut Self>) -> QString;

        #[qinvokable]
        fn start_service(self: Pin<&mut Self>, name: &QString);

        #[qinvokable]
        fn stop_service(self: Pin<&mut Self>, name: &QString);

        #[qinvokable]
        fn restart_service(self: Pin<&mut Self>, name: &QString);

        #[qinvokable]
        fn enable_service(self: Pin<&mut Self>, name: &QString);

        #[qinvokable]
        fn disable_service(self: Pin<&mut Self>, name: &QString);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct ServiceBridgeRust {
    service_name: QString,
    status: QString,
    active: bool,
    enabled: bool,
    service_count: i32,
}

impl qobject::ServiceBridge {
    pub fn load_services(mut self: Pin<&mut Self>) -> QString {
        let services = xarph_sdk::service_manager::ServiceManager::list_services();
        let count = services.len() as i32;
        self.as_mut().set_service_count(count);

        let lines: Vec<String> = services
            .iter()
            .map(|s| {
                format!(
                    "{}|{}|{}|{}",
                    s.name, s.status, s.enabled, s.description
                )
            })
            .collect();
        QString::from(&lines.join("\n"))
    }

    pub fn start_service(self: Pin<&mut Self>, name: &QString) {
        let name_str = name.to_string();
        std::thread::spawn(move || {
            let _ = xarph_sdk::service_manager::ServiceManager::start(&name_str);
        });
    }

    pub fn stop_service(self: Pin<&mut Self>, name: &QString) {
        let name_str = name.to_string();
        std::thread::spawn(move || {
            let _ = xarph_sdk::service_manager::ServiceManager::stop(&name_str);
        });
    }

    pub fn restart_service(self: Pin<&mut Self>, name: &QString) {
        let name_str = name.to_string();
        std::thread::spawn(move || {
            let _ = xarph_sdk::service_manager::ServiceManager::restart(&name_str);
        });
    }

    pub fn enable_service(self: Pin<&mut Self>, name: &QString) {
        let name_str = name.to_string();
        std::thread::spawn(move || {
            let _ = xarph_sdk::service_manager::ServiceManager::enable(&name_str);
        });
    }

    pub fn disable_service(self: Pin<&mut Self>, name: &QString) {
        let name_str = name.to_string();
        std::thread::spawn(move || {
            let _ = xarph_sdk::service_manager::ServiceManager::disable(&name_str);
        });
    }
}
