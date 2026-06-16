//! Xarph Services Manager
//!
//! Reads real systemd user services via systemctl.

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, ColumnView, ColumnViewColumn, CssProvider, Label,
    ListItem, Orientation, ScrolledWindow, SignalListItemFactory, SingleSelection, StringList, gdk,
};

const APP_CSS: &str = r#"
window { background-color: rgba(18, 18, 26, 0.98); }
.service-manager { padding: 24px; }
.service-title { font-size: 20px; font-weight: 700; color: rgba(230, 230, 240, 0.95); margin-bottom: 12px; }
.service-controls { margin-bottom: 12px; gap: 8px; }
.service-controls button {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 8px 16px;
    color: rgba(220, 220, 235, 0.9);
    font-size: 13px;
}
.service-controls button:hover {
    background: rgba(140, 110, 255, 0.12);
    border-color: rgba(140, 110, 255, 0.3);
}
columnview { background: rgba(255, 255, 255, 0.02); border: 1px solid rgba(255, 255, 255, 0.05); border-radius: 10px; }
columnview > header > button { background: rgba(255, 255, 255, 0.04); border-bottom: 1px solid rgba(255, 255, 255, 0.06); color: rgba(200, 200, 215, 0.7); font-weight: 600; font-size: 12px; }
columnview > listview > row { border-bottom: 1px solid rgba(255, 255, 255, 0.03); color: rgba(220, 220, 235, 0.88); }
columnview > listview > row:hover { background: rgba(140, 110, 255, 0.06); }
"#;

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub status: String,
    pub sub_status: String,
    pub enabled: bool,
}

fn fetch_services() -> Vec<ServiceInfo> {
    let mut services = Vec::new();

    if let Ok(out) = std::process::Command::new("systemctl")
        .args([
            "--user",
            "list-units",
            "--type=service",
            "--all",
            "--no-legend",
            "--no-pager",
            "--plain",
            "--no-recursive",
        ])
        .output()
    {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                continue;
            }
            let name = parts[0]
                .strip_suffix(".service")
                .unwrap_or(parts[0])
                .to_string();
            let status = parts[2].to_string();
            let sub_status = parts[3].to_string();

            services.push(ServiceInfo {
                name,
                description: String::new(),
                status,
                sub_status,
                enabled: false,
            });
        }
    }

    if let Ok(out) = std::process::Command::new("systemctl")
        .args([
            "--user",
            "list-unit-files",
            "--type=service",
            "--no-legend",
            "--no-pager",
            "--plain",
        ])
        .output()
    {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let unit_name = parts[0]
                    .strip_suffix(".service")
                    .unwrap_or(parts[0]);
                let enabled = parts[1] == "enabled";
                if let Some(svc) = services.iter_mut().find(|s| s.name == unit_name) {
                    svc.enabled = enabled;
                }
            }
        }
    }

    for svc in &mut services {
        if let Ok(out) = std::process::Command::new("systemctl")
            .args(["--user", "show", &svc.name, "--property=Description"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                if let Some(desc) = line.strip_prefix("Description=") {
                    svc.description = desc.to_string();
                }
            }
        }
    }

    services
}

fn get_selected_service(selection: &SingleSelection) -> Option<String> {
    let idx = selection.selected();
    let model = selection.model()?;
    let obj = model.item(idx)?;
    let sobj = obj.downcast_ref::<gtk4::StringObject>()?;
    let name = sobj.string().split('|').next()?.to_string();
    Some(name)
}

fn systemctl_action(action: &str, service: &str) -> bool {
    std::process::Command::new("systemctl")
        .args(["--user", action, service])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn main() {
    let application = Application::builder()
        .application_id("com.xarph.services")
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

        let services = fetch_services();
        let container = Box::new(Orientation::Vertical, 10);
        container.add_css_class("service-manager");

        let title_label = Label::new(Some("Service Manager"));
        title_label.add_css_class("service-title");
        container.append(&title_label);

        let model = StringList::new(&[]);
        for s in &services {
            model.append(&format!(
                "{}|{}|{}|{}|{}",
                s.name, s.description, s.status, s.sub_status, s.enabled
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

        column_view.append_column(&make_col("Name", 0));
        column_view.append_column(&make_col("Description", 1));
        column_view.append_column(&make_col("Active", 2));
        column_view.append_column(&make_col("Sub", 3));
        column_view.append_column(&make_col("Enabled", 4));

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&column_view));
        scrolled.set_vexpand(true);
        container.append(&scrolled);

        // Buttons
        let button_box = Box::new(Orientation::Horizontal, 8);
        button_box.add_css_class("service-controls");

        let refresh_btn = Button::with_label("Refresh");
        let start_btn = Button::with_label("Start");
        let stop_btn = Button::with_label("Stop");
        let restart_btn = Button::with_label("Restart");

        button_box.append(&refresh_btn);
        button_box.append(&start_btn);
        button_box.append(&stop_btn);
        button_box.append(&restart_btn);
        container.prepend(&button_box);

        // Refresh: re-fetch and rebuild the list
        let model_clone = model.clone();
        refresh_btn.connect_clicked(move |_| {
            let services = fetch_services();
            // Remove all existing items (from end to start)
            while model_clone.n_items() > 0 {
                model_clone.remove(model_clone.n_items() - 1);
            }
            for s in &services {
                model_clone.append(&format!(
                    "{}|{}|{}|{}|{}",
                    s.name, s.description, s.status, s.sub_status, s.enabled
                ));
            }
        });

        // Start
        let sel = selection_model.clone();
        start_btn.connect_clicked(move |_| {
            if let Some(name) = get_selected_service(&sel) {
                let _ = systemctl_action("start", &name);
            }
        });

        // Stop
        let sel = selection_model.clone();
        stop_btn.connect_clicked(move |_| {
            if let Some(name) = get_selected_service(&sel) {
                let _ = systemctl_action("stop", &name);
            }
        });

        // Restart
        let sel = selection_model.clone();
        restart_btn.connect_clicked(move |_| {
            if let Some(name) = get_selected_service(&sel) {
                let _ = systemctl_action("restart", &name);
            }
        });

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Xarph Service Manager")
            .default_width(850)
            .default_height(550)
            .child(&container)
            .build();

        window.present();
    });

    application.run();
}
