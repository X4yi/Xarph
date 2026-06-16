//! Xarph Process Administrator
//!
//! Reads real process data from /proc.

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, ColumnView, ColumnViewColumn, CssProvider, Label,
    ListItem, Orientation, ScrolledWindow, SignalListItemFactory, SingleSelection, StringList, gdk,
    glib,
};

const APP_CSS: &str = r#"
window { background-color: rgba(18, 18, 26, 0.98); }
.process-admin { padding: 24px; }
.process-title { font-size: 20px; font-weight: 700; color: rgba(230, 230, 240, 0.95); margin-bottom: 12px; }
.process-controls { margin-bottom: 12px; gap: 8px; }
.process-controls button {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 8px 16px;
    color: rgba(220, 220, 235, 0.9);
    font-size: 13px;
}
.process-controls button:hover {
    background: rgba(140, 110, 255, 0.12);
    border-color: rgba(140, 110, 255, 0.3);
}
columnview { background: rgba(255, 255, 255, 0.02); border: 1px solid rgba(255, 255, 255, 0.05); border-radius: 10px; }
columnview > header > button { background: rgba(255, 255, 255, 0.04); border-bottom: 1px solid rgba(255, 255, 255, 0.06); color: rgba(200, 200, 215, 0.7); font-weight: 600; font-size: 12px; }
columnview > listview > row { border-bottom: 1px solid rgba(255, 255, 255, 0.03); color: rgba(220, 220, 235, 0.88); }
columnview > listview > row:hover { background: rgba(140, 110, 255, 0.06); }
"#;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub user: String,
    pub cpu_usage: f32,
    pub memory_kb: u64,
    pub status: String,
}

fn fetch_processes() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();

    let proc_dir = match std::fs::read_dir("/proc") {
        Ok(d) => d,
        Err(_) => return processes,
    };

    for entry in proc_dir.flatten() {
        let name = entry.file_name();
        let pid_str = match name.to_str() {
            Some(s) => s,
            None => continue,
        };
        let pid: u32 = match pid_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let stat_path = format!("/proc/{pid}/stat");
        let stat = match std::fs::read_to_string(&stat_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let comm_end = stat.rfind(')').unwrap_or(0);
        let fields: Vec<&str> = stat[comm_end + 2..].split_whitespace().collect();
        if fields.len() < 20 {
            continue;
        }

        let state = fields[0];
        let utime: u64 = fields[11].parse().unwrap_or(0);
        let stime: u64 = fields[12].parse().unwrap_or(0);
        let cpu_ticks = utime + stime;

        let status_path = format!("/proc/{pid}/status");
        let status_text = std::fs::read_to_string(&status_path).unwrap_or_default();

        let vm_rss: u64 = status_text
            .lines()
            .find(|l| l.starts_with("VmRSS:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let uid_line = status_text
            .lines()
            .find(|l| l.starts_with("Uid:"));
        let uid = uid_line
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0);

        let user = resolve_uid(uid);
        let comm = stat[1..comm_end].to_string();

        let clk_tck: u64 = 100;
        let uptime_secs = std::fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|s| s.split_whitespace().next()?.parse::<f64>().ok())
            .unwrap_or(1.0);
        let cpu_secs = cpu_ticks as f64 / clk_tck as f64;
        let cpu_pct = if uptime_secs > 0.0 {
            (cpu_secs / uptime_secs * 100.0) as f32
        } else {
            0.0
        };

        let status_char = match state {
            "R" => "Running",
            "S" => "Sleeping",
            "D" => "Disk Sleep",
            "Z" => "Zombie",
            "T" => "Stopped",
            "t" => "Tracing",
            "I" => "Idle",
            _ => state,
        };

        processes.push(ProcessInfo {
            pid,
            name: comm,
            user,
            cpu_usage: cpu_pct,
            memory_kb: vm_rss,
            status: status_char.to_string(),
        });
    }

    processes.sort_by(|a, b| b.memory_kb.cmp(&a.memory_kb));
    processes
}

fn resolve_uid(uid: u32) -> String {
    if let Ok(passwd) = std::fs::read_to_string("/etc/passwd") {
        for line in passwd.lines() {
            let fields: Vec<&str> = line.split(':').collect();
            if fields.len() >= 3 {
                if let Ok(fuid) = fields[2].parse::<u32>() {
                    if fuid == uid {
                        return fields[0].to_string();
                    }
                }
            }
        }
    }
    format!("{uid}")
}

fn get_selected_pid(selection: &SingleSelection) -> Option<u32> {
    let idx = selection.selected();
    let model = selection.model()?;
    let obj = model.item(idx)?;
    let sobj = obj.downcast_ref::<gtk4::StringObject>()?;
    let pid = sobj.string().split('|').next()?.parse::<u32>().ok()?;
    Some(pid)
}

fn main() {
    let application = Application::builder()
        .application_id("com.xarph.process-admin")
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

        let processes = fetch_processes();
        let container = Box::new(Orientation::Vertical, 10);
        container.add_css_class("process-admin");

        let title_label = Label::new(Some(&format!("Processes ({})", processes.len())));
        title_label.add_css_class("process-title");
        container.append(&title_label);

        let model = StringList::new(&[]);
        for p in &processes {
            model.append(&format!(
                "{}|{}|{}|{:.1}|{:.0} KB|{}",
                p.pid, p.name, p.user, p.cpu_usage, p.memory_kb, p.status
            ));
        }

        let selection_model = SingleSelection::new(Some(model.clone()));
        let column_view = ColumnView::new(Some(selection_model.clone()));

        let make_col = |title: &str, idx: usize| {
            let title = title.to_string();
            let factory = SignalListItemFactory::new();
            factory.connect_setup(move |_factory, item| {
                let item = item.downcast_ref::<ListItem>().unwrap();
                let label = Label::builder().xalign(0.0).build();
                item.set_child(Some(&label));
            });
            factory.connect_bind(move |_factory, item| {
                let item = item.downcast_ref::<ListItem>().unwrap();
                let label = item.child().and_downcast::<Label>().unwrap();
                let obj = item.item().and_downcast::<gtk4::StringObject>().unwrap();
                let val = obj
                    .string()
                    .as_str()
                    .split('|')
                    .nth(idx)
                    .unwrap_or("")
                    .to_string();
                label.set_text(&val);
            });
            let col = ColumnViewColumn::new(Some(&title), Some(factory));
            col.set_expand(true);
            col
        };

        column_view.append_column(&make_col("PID", 0));
        column_view.append_column(&make_col("Name", 1));
        column_view.append_column(&make_col("User", 2));
        column_view.append_column(&make_col("CPU %", 3));
        column_view.append_column(&make_col("Memory", 4));
        column_view.append_column(&make_col("Status", 5));

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&column_view));
        scrolled.set_vexpand(true);
        container.append(&scrolled);

        // Buttons
        let button_box = Box::new(Orientation::Horizontal, 8);
        button_box.add_css_class("process-controls");

        let refresh_btn = Button::with_label("Refresh");
        let kill_btn = Button::with_label("Kill Selected");

        button_box.append(&refresh_btn);
        button_box.append(&kill_btn);
        container.prepend(&button_box);

        // Refresh
        let model_clone = model.clone();
        let title_clone = title_label.clone();
        refresh_btn.connect_clicked(move |_| {
            let processes = fetch_processes();
            title_clone.set_text(&format!("Processes ({})", processes.len()));
            while model_clone.n_items() > 0 {
                model_clone.remove(model_clone.n_items() - 1);
            }
            for p in &processes {
                model_clone.append(&format!(
                    "{}|{}|{}|{:.1}|{:.0} KB|{}",
                    p.pid, p.name, p.user, p.cpu_usage, p.memory_kb, p.status
                ));
            }
        });

        // Kill
        let sel = selection_model.clone();
        kill_btn.connect_clicked(move |_| {
            if let Some(pid) = get_selected_pid(&sel) {
                unsafe {
                    libc::kill(pid as i32, libc::SIGTERM);
                }
            }
        });

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Xarph Process Administrator")
            .default_width(850)
            .default_height(550)
            .child(&container)
            .build();

        window.present();
    });

    application.run();
}
