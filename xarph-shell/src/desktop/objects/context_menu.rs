use gio;
use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{Box, Button, GestureClick, Label, Orientation, Popover, Separator, Widget};

use super::ObjectData;

fn open_path(path: &str) {
    let _ = std::process::Command::new("xdg-open").arg(path).spawn();
}

fn open_in_terminal(path: &str) {
    let terminal = std::env::var("TERMINAL").unwrap_or_else(|_| "xterm".into());
    let _ = std::process::Command::new(&terminal).arg(path).spawn();
}

/// Build and show a context menu for a desktop object.
/// Attaches a right-click gesture to `target` that displays the popover.
pub fn attach_context_menu(target: &Widget, data: ObjectData) {
    let rclick = GestureClick::new();
    rclick.set_button(3);

    let target_clone = target.clone();
    rclick.connect_pressed(move |_, _, x, y| {
        let popover = build_menu_for_type(&data);
        popover.set_parent(&target_clone);
        popover.set_pointing_to(Some(&gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
        popover.popup();

        popover.connect_closed(move |p| {
            p.unparent();
        });
    });

    target.add_controller(rclick);
}

fn build_menu_for_type(data: &ObjectData) -> Popover {
    match data {
        ObjectData::Folder { .. } => build_folder_menu(data),
        ObjectData::File { .. } => build_file_menu(data),
        ObjectData::Application { .. } => build_application_menu(data),
        ObjectData::Project { .. } => build_project_menu(data),
        ObjectData::Shortcut { .. } => build_shortcut_menu(data),
        ObjectData::Widget { .. } => build_widget_menu(data),
    }
}

fn make_popover() -> Popover {
    let popover = Popover::new();
    popover.set_autohide(true);
    popover.add_css_class("context-menu");
    popover
}

fn add_menu_item(vbox: &Box, label: &str, action: impl Fn() + 'static) {
    let btn = Button::builder()
        .label(label)
        .css_classes(["menu-item"])
        .halign(gtk4::Align::Fill)
        .build();
    btn.connect_clicked(move |_| action());
    vbox.append(&btn);
}

fn add_separator(vbox: &Box) {
    vbox.append(&Separator::new(Orientation::Horizontal));
}

fn add_info_label(vbox: &Box, text: &str) {
    let label = Label::builder()
        .label(text)
        .css_classes(["menu-info"])
        .xalign(0.0)
        .selectable(true)
        .build();
    vbox.append(&label);
}

fn build_folder_menu(data: &ObjectData) -> Popover {
    let popover = make_popover();
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.add_css_class("menu-content");

    add_info_label(&vbox, &format!("📁 {}", data.name()));
    add_separator(&vbox);

    let path = match data {
        ObjectData::Folder { path, .. } => path.clone(),
        _ => return popover,
    };
    let p = path.clone();
    add_menu_item(&vbox, "Open", move || open_path(&p));
    let p = path.clone();
    add_menu_item(&vbox, "Open in Terminal", move || open_in_terminal(&p));
    add_separator(&vbox);
    add_menu_item(&vbox, "Properties", || {});

    popover.set_child(Some(&vbox));
    popover
}

fn build_file_menu(data: &ObjectData) -> Popover {
    let popover = make_popover();
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.add_css_class("menu-content");

    add_info_label(&vbox, &format!("📄 {}", data.name()));
    add_separator(&vbox);

    let path = match data {
        ObjectData::File { path, .. } => path.clone(),
        _ => return popover,
    };
    let p = path.clone();
    add_menu_item(&vbox, "Open", move || open_path(&p));
    add_separator(&vbox);
    add_menu_item(&vbox, "Properties", || {});

    popover.set_child(Some(&vbox));
    popover
}

fn build_application_menu(data: &ObjectData) -> Popover {
    let popover = make_popover();
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.add_css_class("menu-content");

    add_info_label(&vbox, &format!("▶ {}", data.name()));
    add_separator(&vbox);

    let desktop_file = match data {
        ObjectData::Application { desktop_file, .. } => desktop_file.clone(),
        _ => return popover,
    };
    let df = desktop_file.clone();
    add_menu_item(&vbox, "Open", move || {
        use gio::prelude::AppInfoExt;
        let app = gio::DesktopAppInfo::new(&df).or_else(|| gio::DesktopAppInfo::from_filename(&df));
        if let Some(app) = app {
            let _ = app.launch(&[], None::<&gio::AppLaunchContext>);
        }
    });
    add_separator(&vbox);
    add_menu_item(&vbox, "Properties", || {});

    popover.set_child(Some(&vbox));
    popover
}

fn build_project_menu(data: &ObjectData) -> Popover {
    let popover = make_popover();
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.add_css_class("menu-content");

    add_info_label(&vbox, &format!("📦 {}", data.name()));
    add_separator(&vbox);

    let path = match data {
        ObjectData::Project { path, .. } => path.clone(),
        _ => return popover,
    };
    let p = path.clone();
    add_menu_item(&vbox, "Open in Terminal", move || open_in_terminal(&p));
    add_separator(&vbox);
    add_menu_item(&vbox, "Properties", || {});

    popover.set_child(Some(&vbox));
    popover
}

fn build_shortcut_menu(data: &ObjectData) -> Popover {
    let popover = make_popover();
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.add_css_class("menu-content");

    add_info_label(&vbox, &format!("🔗 {}", data.name()));
    add_separator(&vbox);

    let target = match data {
        ObjectData::Shortcut { target, .. } => target.clone(),
        _ => return popover,
    };
    let t = target.clone();
    add_menu_item(&vbox, "Open Target", move || open_path(&t));
    add_separator(&vbox);
    add_menu_item(&vbox, "Properties", || {});

    popover.set_child(Some(&vbox));
    popover
}

fn build_widget_menu(data: &ObjectData) -> Popover {
    let popover = make_popover();
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.add_css_class("menu-content");

    add_info_label(&vbox, &format!("🧩 {}", data.name()));
    add_separator(&vbox);

    add_menu_item(&vbox, "Delete", || {});
    add_separator(&vbox);
    add_menu_item(&vbox, "Configure", || {});

    popover.set_child(Some(&vbox));
    popover
}
