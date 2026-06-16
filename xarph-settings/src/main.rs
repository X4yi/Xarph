mod wallpaper_gallery;
mod wallpaper_settings;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, CssProvider, Label, ListBox, ListBoxRow,
    Orientation, Scale, ScrolledWindow, SelectionMode, Stack, StackSidebar, Switch, gdk, glib,
};
use std::fs;
use std::path::PathBuf;
use xarph_sdk::config::{KeybindConfig, PanelWidgetConfig, WidgetKind, XarphConfig};

const SETTINGS_CSS: &str = r#"
/* ═══════════════════════════════════════════════════════════════
   Xarph Settings — Application Stylesheet
   ═══════════════════════════════════════════════════════════════ */

window {
    background-color: rgba(18, 18, 26, 0.98);
}

stacksidebar {
    background: rgba(22, 22, 32, 0.95);
    border-right: 1px solid rgba(255, 255, 255, 0.05);
}

stacksidebar list {
    background: transparent;
}

stacksidebar list row {
    padding: 8px 16px;
    border-radius: 8px;
    margin: 2px 8px;
}

stacksidebar list row:hover {
    background: rgba(255, 255, 255, 0.05);
}

stacksidebar list row:selected {
    background: rgba(140, 110, 255, 0.15);
}

.settings-title {
    font-size: 18px;
    font-weight: 700;
    color: rgba(230, 230, 240, 0.95);
}

.settings-subtitle {
    font-size: 13px;
    color: rgba(200, 200, 215, 0.6);
}

.wallpaper-gallery-item {
    border-radius: 10px;
    overflow: hidden;
    transition: all 160ms ease;
}

.wallpaper-gallery-item:hover {
    box-shadow: 0 0 0 2px rgba(140, 110, 255, 0.5);
}

.wallpaper-gallery-label {
    font-size: 11px;
    color: rgba(200, 200, 215, 0.75);
    padding: 4px 0;
}

.suggested-action {
    background: rgba(140, 110, 255, 0.85);
    color: white;
    border: none;
    border-radius: 8px;
    padding: 8px 16px;
    font-weight: 600;
    transition: all 160ms ease;
}

.suggested-action:hover {
    background: rgba(155, 125, 255, 0.95);
}

.suggested-action:active {
    background: rgba(125, 95, 240, 0.95);
}

entry {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 8px 12px;
    color: rgba(220, 220, 235, 0.92);
    transition: all 160ms ease;
}

entry:focus {
    border-color: rgba(140, 110, 255, 0.4);
    box-shadow: 0 0 0 2px rgba(140, 110, 255, 0.1);
}

combobox {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 8px;
    padding: 6px 12px;
    color: rgba(220, 220, 235, 0.92);
}

list.bordered-list, listbox, .boxed-list {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 10px;
}

list.bordered-list row, listbox row, .boxed-list row {
    padding: 8px 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
}

list.bordered-list row:last-child, listbox row:last-child, .boxed-list row:last-child {
    border-bottom: none;
}

settings-label, label {
    color: rgba(210, 210, 225, 0.88);
}

.keybind-row {
    padding: 8px 12px;
}

.keybind-label {
    font-weight: 600;
    min-width: 180px;
}

.keybind-key {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 6px;
    padding: 4px 10px;
    font-family: monospace;
    font-size: 13px;
    color: rgba(170, 140, 255, 1.0);
}
"#;

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(SETTINGS_CSS);

    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn get_available_themes() -> Vec<String> {
    let mut themes = Vec::new();
    let dirs = vec![
        PathBuf::from("/usr/share/themes"),
        dirs::home_dir().unwrap_or_default().join(".themes"),
        dirs::data_dir().unwrap_or_default().join("themes"),
    ];

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let has_gtk = entry.path().join("gtk-3.0").exists()
                            || entry.path().join("gtk-4.0").exists();
                        if has_gtk {
                            if let Some(name) = entry.file_name().to_str() {
                                themes.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    themes.sort();
    themes.dedup();
    themes
}

// ── General Page ───────────────────────────────────────────────────────

fn build_general_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(24);
    vbox.set_margin_start(24);
    vbox.set_margin_end(24);

    let title = Label::builder()
        .label("<b>General Settings</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    vbox.append(&title);

    // GTK Theme
    let theme_label = Label::builder()
        .label("GTK Theme")
        .xalign(0.0)
        .build();
    vbox.append(&theme_label);

    let theme_list = ListBox::new();
    theme_list.set_selection_mode(SelectionMode::Single);
    theme_list.add_css_class("boxed-list");

    let current_config = XarphConfig::load();
    let themes = get_available_themes();

    for theme in themes {
        let row = ListBoxRow::new();
        let label = Label::builder()
            .label(&theme)
            .xalign(0.0)
            .margin_start(12)
            .margin_top(12)
            .margin_bottom(12)
            .build();
        row.set_child(Some(&label));
        theme_list.append(&row);

        if Some(theme) == current_config.theme {
            theme_list.select_row(Some(&row));
        }
    }

    theme_list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            if let Some(label) = row.child().and_downcast::<Label>() {
                let theme_name = label.label().to_string();
                let mut config = XarphConfig::load();
                config.theme = Some(theme_name.clone());
                let _ = config.save();

                if let Some(settings) = gtk4::Settings::default() {
                    settings.set_gtk_theme_name(Some(&theme_name));
                }
            }
        }
    });

    vbox.append(&theme_list);

    // Corner Radius
    let radius_label = Label::builder()
        .label("Corner Radius (px)")
        .xalign(0.0)
        .build();
    vbox.append(&radius_label);

    let radius_scale = Scale::with_range(Orientation::Horizontal, 0.0, 24.0, 1.0);
    radius_scale.set_value(10.0);
    radius_scale.set_hexpand(true);
    vbox.append(&radius_scale);

    vbox
}

// ── Panel Page ─────────────────────────────────────────────────────────

fn build_panel_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(24);
    vbox.set_margin_start(24);
    vbox.set_margin_end(24);

    let title = Label::builder()
        .label("<b>Panel Widgets</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    vbox.append(&title);

    let subtitle = Label::builder()
        .label("Toggle which widgets appear in the top panel.")
        .xalign(0.0)
        .build();
    vbox.append(&subtitle);

    let config = XarphConfig::load();

    let widgets = [
        ("Start Button", WidgetKind::StartButton, true),
        ("Workspaces", WidgetKind::Workspaces, true),
        ("Clock", WidgetKind::Clock, true),
        ("System Tray", WidgetKind::Tray, true),
    ];

    for (label, kind, default_visible) in &widgets {
        let row = Box::new(Orientation::Horizontal, 12);
        row.set_margin_top(4);
        row.set_margin_bottom(4);

        let lbl = Label::builder()
            .label(*label)
            .xalign(0.0)
            .hexpand(true)
            .build();
        row.append(&lbl);

        let switch = Switch::new();
        // Check if widget exists in panel config and is visible
        let visible = config
            .shell
            .panels
            .first()
            .map(|p| {
                p.widgets
                    .iter()
                    .find(|w| std::mem::discriminant(&w.kind) == std::mem::discriminant(kind))
                    .map(|w| w.visible)
                    .unwrap_or(*default_visible)
            })
            .unwrap_or(*default_visible);
        switch.set_active(visible);

        let kind_clone = kind.clone();
        switch.connect_state_set(move |_, is_on| {
            let mut config = XarphConfig::load();
            if let Some(panel) = config.shell.panels.first_mut() {
                if let Some(widget) = panel
                    .widgets
                    .iter_mut()
                    .find(|w| std::mem::discriminant(&w.kind) == std::mem::discriminant(&kind_clone))
                {
                    widget.visible = is_on;
                }
            }
            let _ = config.save();
            glib::Propagation::Proceed
        });

        row.append(&switch);
        vbox.append(&row);
    }

    vbox
}

// ── Theme Page ─────────────────────────────────────────────────────────

fn build_theme_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(24);
    vbox.set_margin_start(24);
    vbox.set_margin_end(24);

    let title = Label::builder()
        .label("<b>Theme & Wallpaper</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    vbox.append(&title);

    let config = XarphConfig::load();

    // Wallpaper Type
    let type_label = Label::builder().label("Wallpaper Type:").xalign(0.0).build();
    vbox.append(&type_label);

    let type_combo = gtk4::ComboBoxText::new();
    type_combo.append_text("Image");
    type_combo.append_text("Solid Color");
    match &config.wallpaper_global {
        xarph_sdk::config::WallpaperConfig::Image { .. } => type_combo.set_active(Some(0)),
        xarph_sdk::config::WallpaperConfig::Color { .. } => type_combo.set_active(Some(1)),
    }
    vbox.append(&type_combo);

    // Path entry
    let path_label = Label::builder().label("Wallpaper Path:").xalign(0.0).build();
    vbox.append(&path_label);

    let path_box = Box::new(Orientation::Horizontal, 4);
    let path_entry = gtk4::Entry::new();
    path_entry.set_hexpand(true);
    if let xarph_sdk::config::WallpaperConfig::Image { ref path, .. } = config.wallpaper_global {
        path_entry.set_text(path);
    }
    let browse_btn = Button::with_label("Browse");
    path_box.append(&path_entry);
    path_box.append(&browse_btn);
    vbox.append(&path_box);

    // Browse button handler
    let path_entry_c = path_entry.clone();
    browse_btn.connect_clicked(move |_| {
        let dialog = gtk4::FileChooserDialog::builder()
            .title("Select Wallpaper")
            .action(gtk4::FileChooserAction::Open)
            .build();

        dialog.add_button("Cancel", gtk4::ResponseType::Cancel);
        dialog.add_button("Open", gtk4::ResponseType::Accept);

        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Images"));
        filter.add_mime_type("image/png");
        filter.add_mime_type("image/jpeg");
        filter.add_mime_type("image/webp");
        dialog.add_filter(&filter);

        let entry_clone = path_entry_c.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        entry_clone.set_text(&path.to_string_lossy());
                    }
                }
            }
            dialog.close();
        });

        dialog.present();
    });

    // Color entry
    let color_label = Label::builder().label("Color (hex):").xalign(0.0).build();
    vbox.append(&color_label);

    let color_entry = gtk4::Entry::new();
    if let xarph_sdk::config::WallpaperConfig::Color { ref hex } = config.wallpaper_global {
        color_entry.set_text(hex);
    }
    vbox.append(&color_entry);

    // Mode
    let mode_label = Label::builder().label("Mode:").xalign(0.0).build();
    vbox.append(&mode_label);

    let mode_combo = gtk4::ComboBoxText::new();
    mode_combo.append_text("Fill");
    mode_combo.append_text("Fit");
    mode_combo.append_text("Stretch");
    mode_combo.append_text("Center");
    mode_combo.append_text("Tile");
    if let xarph_sdk::config::WallpaperConfig::Image { ref mode, .. } = config.wallpaper_global {
        let idx = match mode {
            xarph_sdk::config::WallpaperMode::Fill => 0,
            xarph_sdk::config::WallpaperMode::Fit => 1,
            xarph_sdk::config::WallpaperMode::Stretch => 2,
            xarph_sdk::config::WallpaperMode::Center => 3,
            xarph_sdk::config::WallpaperMode::Tile => 4,
        };
        mode_combo.set_active(Some(idx));
    } else {
        mode_combo.set_active(Some(0));
    }
    vbox.append(&mode_combo);

    // Status label
    let status_label = Label::new(None);
    status_label.set_opacity(0.0);
    vbox.append(&status_label);

    // Save button
    let save_btn = Button::with_label("Apply Wallpaper");
    save_btn.add_css_class("suggested-action");

    let path_entry_c = path_entry.clone();
    let color_entry_c = color_entry.clone();
    let type_combo_c = type_combo.clone();
    let mode_combo_c = mode_combo.clone();
    let status_clone = status_label.clone();
    save_btn.connect_clicked(move |_| {
        let mut config = XarphConfig::load();
        let selected_type = type_combo_c.active().unwrap_or(0);

        let wallpaper = if selected_type == 0 {
            let path = path_entry_c.text().to_string();
            let mode_idx = mode_combo_c.active().unwrap_or(0);
            let mode = match mode_idx {
                0 => xarph_sdk::config::WallpaperMode::Fill,
                1 => xarph_sdk::config::WallpaperMode::Fit,
                2 => xarph_sdk::config::WallpaperMode::Stretch,
                3 => xarph_sdk::config::WallpaperMode::Center,
                _ => xarph_sdk::config::WallpaperMode::Fill,
            };
            xarph_sdk::config::WallpaperConfig::Image { path, mode }
        } else {
            let hex = color_entry_c.text().to_string();
            xarph_sdk::config::WallpaperConfig::Color { hex }
        };

        config.wallpaper_global = wallpaper;
        let _ = config.save();

        status_clone.set_text("Applied!");
        status_clone.set_opacity(1.0);
        let sc = status_clone.clone();
        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            sc.set_opacity(0.0);
            glib::ControlFlow::Break
        });
    });

    vbox.append(&save_btn);

    // Wallpaper gallery
    let separator = gtk4::Separator::new(Orientation::Horizontal);
    separator.set_margin_top(8);
    separator.set_margin_bottom(8);
    vbox.append(&separator);

    let gallery = wallpaper_gallery::build_wallpaper_gallery();
    gallery.set_vexpand(true);
    vbox.append(&gallery);

    vbox
}

// ── Shortcuts Page ─────────────────────────────────────────────────────

fn build_shortcuts_page() -> Box {
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(24);
    vbox.set_margin_start(24);
    vbox.set_margin_end(24);

    let title = Label::builder()
        .label("<b>Keyboard Shortcuts</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    vbox.append(&title);

    let subtitle = Label::builder()
        .label("Keybinds are configured in the compositor config (KDL). These are reference values.")
        .xalign(0.0)
        .wrap(true)
        .build();
    vbox.append(&subtitle);

    let config = XarphConfig::load();
    let kb = &config.keybind_config;

    let shortcuts = [
        ("Quit Session", &kb.key_quit),
        ("Lock Screen", &kb.key_lock),
        ("Terminal", &kb.key_terminal),
        ("App Launcher", &kb.key_launcher),
        ("Close Window", &kb.key_close_window),
        ("Toggle Floating", &kb.key_toggle_floating),
        ("Toggle Overview", &kb.key_toggle_overview),
        ("Screenshot", &kb.key_screenshot),
        ("Previous Workspace", &kb.key_workspace_prev),
        ("Next Workspace", &kb.key_workspace_next),
        ("Focus Left", &kb.key_focus_left),
        ("Focus Right", &kb.key_focus_right),
        ("Focus Up", &kb.key_focus_up),
        ("Focus Down", &kb.key_focus_down),
        ("Move Left", &kb.key_move_left),
        ("Move Right", &kb.key_move_right),
        ("Move Up", &kb.key_move_up),
        ("Move Down", &kb.key_move_down),
    ];

    let list = ListBox::new();
    list.add_css_class("boxed-list");

    for (label, key) in &shortcuts {
        let row = Box::new(Orientation::Horizontal, 12);
        row.set_margin_top(4);
        row.set_margin_bottom(4);
        row.add_css_class("keybind-row");

        let lbl = Label::builder()
            .label(*label)
            .xalign(0.0)
            .hexpand(true)
            .css_classes(["keybind-label"])
            .build();
        row.append(&lbl);

        let key_label = Label::builder()
            .label(*key)
            .css_classes(["keybind-key"])
            .build();
        row.append(&key_label);

        list.append(&row);
    }

    let scrolled = ScrolledWindow::builder()
        .child(&list)
        .vexpand(true)
        .build();
    vbox.append(&scrolled);

    vbox
}

// ── Main ───────────────────────────────────────────────────────────────

fn main() {
    let application = Application::builder()
        .application_id("com.xarph.settings")
        .build();

    application.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Xarph Settings")
            .default_width(900)
            .default_height(650)
            .build();

        let hbox = Box::new(Orientation::Horizontal, 0);
        let stack = Stack::new();
        stack.set_hexpand(true);
        stack.set_vexpand(true);

        let sidebar = StackSidebar::new();
        sidebar.set_stack(&stack);
        sidebar.set_size_request(220, -1);

        hbox.append(&sidebar);
        hbox.append(&stack);

        // General Page
        let general_box = build_general_page();
        stack.add_titled(&general_box, Some("general"), "General");

        // Panel Page
        let panel_box = build_panel_page();
        stack.add_titled(&panel_box, Some("panel"), "Panel");

        // Theme Page
        let theme_box = build_theme_page();
        stack.add_titled(&theme_box, Some("theme"), "Theme & Wallpaper");

        // Shortcuts Page
        let shortcuts_box = build_shortcuts_page();
        stack.add_titled(&shortcuts_box, Some("shortcuts"), "Shortcuts");

        window.set_child(Some(&hbox));

        // Apply our own theme on startup
        let current_config = XarphConfig::load();
        if let Some(settings) = gtk4::Settings::default() {
            if let Some(theme) = current_config.theme {
                settings.set_gtk_theme_name(Some(&theme));
            }
        }

        window.present();
    });

    application.run();
}
