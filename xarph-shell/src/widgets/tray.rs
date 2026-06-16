use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Image, Label, Orientation, Widget, gdk, glib};
use std::sync::{Arc, Mutex, OnceLock};

use super::ShellWidget;

#[derive(Debug, Clone)]
struct TrayItem {
    bus_name: String,
    icon_name: String,
    title: String,
}

#[derive(Debug, Clone, Default)]
struct TrayState {
    items: Vec<TrayItem>,
}

static TRAY_STATE: OnceLock<Arc<Mutex<TrayState>>> = OnceLock::new();

pub struct TrayWidget {
    pub icon_size: i32,
    pub orientation: Orientation,
}

impl ShellWidget for TrayWidget {
    fn build(&self) -> Widget {
        let state = tray_state();
        ensure_watcher_thread(state.clone());

        let container = GtkBox::new(self.orientation, 4);
        container.add_css_class("tray-widget");
        container.set_size_request(self.icon_size.max(16), -1);

        rebuild(
            &container,
            &state.lock().map(|s| s.items.clone()).unwrap_or_default(),
            self.icon_size,
        );

        let container_clone = container.clone();
        let icon_size = self.icon_size;
        glib::timeout_add_seconds_local(2, move || {
            let items = state.lock().map(|s| s.items.clone()).unwrap_or_default();
            rebuild(&container_clone, &items, icon_size);
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}

fn tray_state() -> Arc<Mutex<TrayState>> {
    TRAY_STATE
        .get_or_init(|| Arc::new(Mutex::new(TrayState::default())))
        .clone()
}

fn rebuild(container: &GtkBox, items: &[TrayItem], icon_size: i32) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }

    if items.is_empty() {
        return;
    }

    let display = match gdk::Display::default() {
        Some(d) => d,
        None => return,
    };
    let icon_theme = gtk4::IconTheme::for_display(&display);

    for item in items {
        let image = if !item.icon_name.is_empty() {
            // Try to look up the icon by name
            if icon_theme.has_icon(&item.icon_name) {
                let img = Image::from_icon_name(&item.icon_name);
                img.set_pixel_size(icon_size);
                img
            } else {
                // Fallback to first letter
                make_fallback_image(&item.title, icon_size)
            }
        } else {
            make_fallback_image(&item.title, icon_size)
        };

        image.set_halign(gtk4::Align::Center);
        image.set_valign(gtk4::Align::Center);

        let button = Button::builder()
            .child(&image)
            .tooltip_text(&item.title)
            .css_classes(vec!["tray-btn".to_string(), "flat".to_string()])
            .build();
        container.append(&button);
    }
}

fn make_fallback_image(title: &str, icon_size: i32) -> Image {
    let first_char = title.chars().next().unwrap_or('?');
    let label = Label::builder()
        .label(&first_char.to_string())
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();
    label.set_size_request(icon_size, icon_size);
    // Wrap label in an Image via from_paintable won't work, so use icon_name fallback
    // For simplicity, use a generic icon
    Image::from_icon_name("application-x-executable")
}

fn ensure_watcher_thread(state: Arc<Mutex<TrayState>>) {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(move || {
        std::thread::spawn(move || run_watcher(state));
    });
}

fn run_watcher(state: Arc<Mutex<TrayState>>) {
    let Ok(connection) = zbus::blocking::Connection::session() else {
        return;
    };

    let _ = connection.request_name("org.kde.StatusNotifierWatcher");

    loop {
        let items = get_tray_items(&connection);
        if let Ok(mut guard) = state.lock() {
            guard.items = items;
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn get_tray_items(connection: &zbus::blocking::Connection) -> Vec<TrayItem> {
    let Ok(proxy) = zbus::blocking::Proxy::new(
        connection,
        "org.kde.StatusNotifierWatcher",
        "/StatusNotifierWatcher",
        "org.kde.StatusNotifierWatcher",
    ) else {
        return Vec::new();
    };

    let Ok(registered) = proxy.get_property::<Vec<String>>("RegisteredStatusNotifierItems") else {
        return Vec::new();
    };

    let mut items = Vec::new();
    for item_path in registered {
        if let Some(item) = get_item_properties(connection, &item_path) {
            items.push(item);
        }
    }
    items
}

fn get_item_properties(connection: &zbus::blocking::Connection, item_path: &str) -> Option<TrayItem> {
    let (bus_name_str, obj_path_str) = if item_path.starts_with('/') {
        ("org.kde.StatusNotifierItem".to_string(), item_path.to_string())
    } else {
        let parts: Vec<&str> = item_path.splitn(2, '/').collect();
        if parts.len() < 2 {
            return None;
        }
        let bus = format!("org.kde.StatusNotifierItem-{}-{}", parts[0], parts[0]);
        (bus, format!("/{}", parts[1]))
    };

    let icon_name;
    let title;
    let id;

    {
        let proxy = zbus::blocking::Proxy::new(
            connection,
            bus_name_str.as_str(),
            obj_path_str.as_str(),
            "org.kde.StatusNotifierItem",
        ).ok()?;

        icon_name = proxy.get_property::<String>("IconName").unwrap_or_default();
        title = proxy.get_property::<String>("Title").unwrap_or_default();
        id = proxy.get_property::<String>("Id").unwrap_or_default();
    }

    let display_name = if !title.is_empty() {
        title
    } else if !id.is_empty() {
        id
    } else {
        obj_path_str.split('/').last().unwrap_or("tray").to_string()
    };

    Some(TrayItem {
        bus_name: bus_name_str,
        icon_name,
        title: display_name,
    })
}
