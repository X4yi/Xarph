/// Process admin bridge: exposes process management to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, pid)]
        #[qproperty(QString, name)]
        #[qproperty(QString, status)]
        #[qproperty(f64, cpu_usage)]
        #[qproperty(u64, memory_usage)]
        #[qproperty(i32, process_count)]
        #[namespace = "xarph"]
        type ProcessBridge = super::ProcessBridgeRust;

        #[qinvokable]
        fn load_processes(self: Pin<&mut Self>) -> QString;

        #[qinvokable]
        fn kill_process(self: Pin<&mut Self>, pid: i32);

        #[qinvokable]
        fn terminate_process(self: Pin<&mut Self>, pid: i32);

        #[qinvokable]
        fn get_process_count(&self) -> i32;

        #[qinvokable]
        fn get_system_stats(&self) -> QString;
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct ProcessBridgeRust {
    pid: i32,
    name: QString,
    status: QString,
    cpu_usage: f64,
    memory_usage: u64,
    process_count: i32,
}

impl qobject::ProcessBridge {
    pub fn load_processes(mut self: Pin<&mut Self>) -> QString {
        let processes = xarph_sdk::process_collector::ProcessCollector::fetch_all();
        let count = processes.len() as i32;
        self.as_mut().set_process_count(count);

        // Return all processes as pipe-delimited string for QML ListModel
        let lines: Vec<String> = processes
            .iter()
            .map(|p| {
                format!(
                    "{}|{}|{}|{}|{}",
                    p.pid,
                    p.name,
                    p.status,
                    p.cpu_usage,
                    p.memory_kb
                )
            })
            .collect();
        QString::from(&lines.join("\n"))
    }

    pub fn kill_process(self: Pin<&mut Self>, pid: i32) {
        xarph_sdk::process_collector::ProcessCollector::kill_force(pid as u32);
    }

    pub fn terminate_process(self: Pin<&mut Self>, pid: i32) {
        xarph_sdk::process_collector::ProcessCollector::terminate(pid as u32);
    }

    pub fn get_process_count(&self) -> i32 {
        *self.process_count()
    }

    pub fn get_system_stats(&self) -> QString {
        let stats = xarph_sdk::process_collector::ProcessCollector::system_stats();
        let used_mb = stats.used_memory_kb() / 1024;
        let total_mb = stats.total_memory_kb / 1024;
        let usage_pct = stats.memory_usage_percent();
        QString::from(&format!(
            "CPU: {:.1}/{:.1}/{:.1} | Mem: {}/{} MB ({:.1}%)",
            stats.load_1m, stats.load_5m, stats.load_15m,
            used_mb, total_mb, usage_pct
        ))
    }
}
