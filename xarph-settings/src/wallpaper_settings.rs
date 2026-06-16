use gtk4::prelude::*;
use gtk4::{
    Box, Button, ComboBoxText, Entry, FileChooserAction, FileChooserDialog, Label, ListBox,
    ListBoxRow, Orientation, PolicyType, ResponseType, ScrolledWindow, SelectionMode, Stack, glib,
};
use xarph_sdk::config::{WallpaperConfig, WallpaperMode, XarphConfig};

pub fn build_wallpaper_settings() -> Box {
    let hbox = Box::new(Orientation::Horizontal, 0);

    // ── Left: Workspace list ─────────────────────────────────────────
    let left_panel = Box::new(Orientation::Vertical, 4);
    left_panel.set_size_request(200, -1);
    left_panel.set_margin_top(10);
    left_panel.set_margin_start(10);
    left_panel.set_margin_end(10);

    let workspace_label = Label::builder()
        .label("<b>Workspaces</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    left_panel.append(&workspace_label);

    let workspace_list = ListBox::new();
    workspace_list.set_selection_mode(SelectionMode::Single);
    workspace_list.add_css_class("boxed-list");
    workspace_list.set_hexpand(true);
    workspace_list.set_vexpand(true);

    for i in 1..=4u8 {
        let row = ListBoxRow::new();
        let label = Label::builder()
            .label(&format!("Workspace {i}"))
            .xalign(0.0)
            .margin_start(10)
            .margin_top(10)
            .margin_bottom(10)
            .build();
        row.set_child(Some(&label));
        workspace_list.append(&row);
    }

    let scrolled_workspaces = ScrolledWindow::new();
    scrolled_workspaces.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled_workspaces.set_child(Some(&workspace_list));

    let add_btn = Button::with_label("+");
    add_btn.add_css_class("suggested-action");

    left_panel.append(&scrolled_workspaces);
    left_panel.append(&add_btn);

    // Global section
    let global_label = Label::builder()
        .label("<b>Global Wallpapers</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    left_panel.append(&global_label);

    let global_btn = Button::with_label("Configure Global");
    left_panel.append(&global_btn);

    // ── Right: Workspace/Global config panel ─────────────────────────
    let right_panel = Box::new(Orientation::Vertical, 0);

    let config_stack = Stack::new();
    config_stack.set_hexpand(true);
    config_stack.set_vexpand(false);

    for i in 1..=4u8 {
        let page = build_workspace_wallpaper_page(i);
        config_stack.add_titled(&page, Some(&format!("ws-{i}")), &format!("Workspace {i}"));
    }

    let global_page = build_global_wallpaper_page();
    config_stack.add_titled(&global_page, Some("global"), "Global");

    config_stack.set_visible_child_name("ws-1");

    let ws_stack = config_stack.clone();
    let global_stack = config_stack.clone();

    workspace_list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let idx = row.index();
            if idx >= 0 {
                ws_stack.set_visible_child_name(&format!("ws-{}", idx + 1));
            }
        }
    });

    global_btn.connect_clicked(move |_| {
        global_stack.set_visible_child_name("global");
    });

    add_btn.connect_clicked(|_| {});

    right_panel.append(&config_stack);

    let separator = gtk4::Separator::new(Orientation::Horizontal);
    separator.set_margin_top(8);
    separator.set_margin_bottom(8);
    right_panel.append(&separator);

    let gallery = crate::wallpaper_gallery::build_wallpaper_gallery();
    gallery.set_vexpand(true);
    right_panel.append(&gallery);

    hbox.append(&left_panel);
    hbox.append(&right_panel);

    hbox
}

fn build_workspace_wallpaper_page(ws_idx: u8) -> Box {
    let config = XarphConfig::load();
    let wallpaper = config.get_wallpaper_for_workspace(ws_idx).clone();

    let vbox = Box::new(Orientation::Vertical, 8);
    vbox.set_margin_top(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    let title = Label::builder()
        .label(&format!("<b>Workspace {ws_idx} Wallpaper</b>"))
        .use_markup(true)
        .xalign(0.0)
        .build();
    vbox.append(&title);

    // Type combobox
    let type_label = Label::builder().label("Type:").xalign(0.0).build();
    vbox.append(&type_label);

    let type_combo = ComboBoxText::new();
    type_combo.append_text("Image");
    type_combo.append_text("Solid Color");
    match wallpaper {
        WallpaperConfig::Image { .. } => type_combo.set_active(Some(0)),
        WallpaperConfig::Color { .. } => type_combo.set_active(Some(1)),
    }
    vbox.append(&type_combo);

    // Path entry
    let path_label = Label::builder().label("Path:").xalign(0.0).build();
    vbox.append(&path_label);

    let path_box = Box::new(Orientation::Horizontal, 4);
    let path_entry = Entry::new();
    path_entry.set_hexpand(true);
    if let WallpaperConfig::Image { ref path, .. } = wallpaper {
        path_entry.set_text(path);
    }
    let browse_btn = Button::with_label("Browse");
    path_box.append(&path_entry);
    path_box.append(&browse_btn);
    vbox.append(&path_box);

    // Browse button handler
    let path_entry_c = path_entry.clone();
    browse_btn.connect_clicked(move |_| {
        let dialog = FileChooserDialog::builder()
            .title("Select Wallpaper")
            .action(FileChooserAction::Open)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Open", ResponseType::Accept);

        // Add image file filters
        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Images"));
        filter.add_mime_type("image/png");
        filter.add_mime_type("image/jpeg");
        filter.add_mime_type("image/webp");
        filter.add_pattern("*.png");
        filter.add_pattern("*.jpg");
        filter.add_pattern("*.jpeg");
        filter.add_pattern("*.webp");
        dialog.add_filter(&filter);

        let all_filter = gtk4::FileFilter::new();
        all_filter.set_name(Some("All Files"));
        all_filter.add_pattern("*");
        dialog.add_filter(&all_filter);

        let entry_clone = path_entry_c.clone();
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
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

    let color_entry = Entry::new();
    if let WallpaperConfig::Color { ref hex } = wallpaper {
        color_entry.set_text(hex);
    }
    vbox.append(&color_entry);

    // Mode combobox
    let mode_label = Label::builder().label("Mode:").xalign(0.0).build();
    vbox.append(&mode_label);

    let mode_combo = ComboBoxText::new();
    mode_combo.append_text("Fill");
    mode_combo.append_text("Fit");
    mode_combo.append_text("Stretch");
    mode_combo.append_text("Center");
    mode_combo.append_text("Tile");
    if let WallpaperConfig::Image { ref mode, .. } = wallpaper {
        let idx = match mode {
            WallpaperMode::Fill => 0,
            WallpaperMode::Fit => 1,
            WallpaperMode::Stretch => 2,
            WallpaperMode::Center => 3,
            WallpaperMode::Tile => 4,
        };
        mode_combo.set_active(Some(idx));
    } else {
        mode_combo.set_active(Some(0));
    }
    vbox.append(&mode_combo);

    // Status label for feedback
    let status_label = Label::new(None);
    status_label.set_opacity(0.0);
    vbox.append(&status_label);

    // Save button
    let save_btn = Button::with_label("Apply to Workspace");
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
                0 => WallpaperMode::Fill,
                1 => WallpaperMode::Fit,
                2 => WallpaperMode::Stretch,
                3 => WallpaperMode::Center,
                _ => WallpaperMode::Fill,
            };
            WallpaperConfig::Image { path, mode }
        } else {
            let hex = color_entry_c.text().to_string();
            WallpaperConfig::Color { hex }
        };

        config.set_workspace_wallpaper(ws_idx, wallpaper);
        let _ = config.save();

        // Show feedback
        status_clone.set_text("Applied!");
        status_clone.set_opacity(1.0);
        let sc = status_clone.clone();
        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            sc.set_opacity(0.0);
            glib::ControlFlow::Break
        });
    });

    vbox.append(&save_btn);

    vbox
}

fn build_global_wallpaper_page() -> Box {
    let config = XarphConfig::load();
    let wallpaper = config.wallpaper_global.clone();

    let vbox = Box::new(Orientation::Vertical, 8);
    vbox.set_margin_top(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    let title = Label::builder()
        .label("<b>Global Wallpaper</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    vbox.append(&title);

    let desc = Label::builder()
        .label("Wallpapers assigned here are used as fallback for workspaces without their own configuration.")
        .xalign(0.0)
        .wrap(true)
        .build();
    vbox.append(&desc);

    let type_label = Label::builder().label("Type:").xalign(0.0).build();
    vbox.append(&type_label);

    let type_combo = ComboBoxText::new();
    type_combo.append_text("Image");
    type_combo.append_text("Solid Color");
    match wallpaper {
        WallpaperConfig::Image { .. } => type_combo.set_active(Some(0)),
        WallpaperConfig::Color { .. } => type_combo.set_active(Some(1)),
    }
    vbox.append(&type_combo);

    let path_label = Label::builder().label("Path:").xalign(0.0).build();
    vbox.append(&path_label);

    let path_box = Box::new(Orientation::Horizontal, 4);
    let path_entry = Entry::new();
    path_entry.set_hexpand(true);
    if let WallpaperConfig::Image { ref path, .. } = wallpaper {
        path_entry.set_text(path);
    }
    let browse_btn = Button::with_label("Browse");
    path_box.append(&path_entry);
    path_box.append(&browse_btn);
    vbox.append(&path_box);

    // Browse button handler
    let path_entry_c = path_entry.clone();
    browse_btn.connect_clicked(move |_| {
        let dialog = FileChooserDialog::builder()
            .title("Select Wallpaper")
            .action(FileChooserAction::Open)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Open", ResponseType::Accept);

        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Images"));
        filter.add_mime_type("image/png");
        filter.add_mime_type("image/jpeg");
        filter.add_mime_type("image/webp");
        filter.add_pattern("*.png");
        filter.add_pattern("*.jpg");
        filter.add_pattern("*.jpeg");
        filter.add_pattern("*.webp");
        dialog.add_filter(&filter);

        let all_filter = gtk4::FileFilter::new();
        all_filter.set_name(Some("All Files"));
        all_filter.add_pattern("*");
        dialog.add_filter(&all_filter);

        let entry_clone = path_entry_c.clone();
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
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

    let color_label = Label::builder().label("Color (hex):").xalign(0.0).build();
    vbox.append(&color_label);

    let color_entry = Entry::new();
    if let WallpaperConfig::Color { ref hex } = wallpaper {
        color_entry.set_text(hex);
    }
    vbox.append(&color_entry);

    let mode_label = Label::builder().label("Mode:").xalign(0.0).build();
    vbox.append(&mode_label);

    let mode_combo = ComboBoxText::new();
    mode_combo.append_text("Fill");
    mode_combo.append_text("Fit");
    mode_combo.append_text("Stretch");
    mode_combo.append_text("Center");
    mode_combo.append_text("Tile");
    if let WallpaperConfig::Image { ref mode, .. } = wallpaper {
        let idx = match mode {
            WallpaperMode::Fill => 0,
            WallpaperMode::Fit => 1,
            WallpaperMode::Stretch => 2,
            WallpaperMode::Center => 3,
            WallpaperMode::Tile => 4,
        };
        mode_combo.set_active(Some(idx));
    } else {
        mode_combo.set_active(Some(0));
    }
    vbox.append(&mode_combo);

    // Status label for feedback
    let status_label = Label::new(None);
    status_label.set_opacity(0.0);
    vbox.append(&status_label);

    let save_btn = Button::with_label("Apply Global");
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
                0 => WallpaperMode::Fill,
                1 => WallpaperMode::Fit,
                2 => WallpaperMode::Stretch,
                3 => WallpaperMode::Center,
                _ => WallpaperMode::Fill,
            };
            WallpaperConfig::Image { path, mode }
        } else {
            let hex = color_entry_c.text().to_string();
            WallpaperConfig::Color { hex }
        };

        config.wallpaper_global = wallpaper;
        let _ = config.save();

        // Show feedback
        status_clone.set_text("Applied!");
        status_clone.set_opacity(1.0);
        let sc = status_clone.clone();
        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            sc.set_opacity(0.0);
            glib::ControlFlow::Break
        });
    });

    vbox.append(&save_btn);

    vbox
}
