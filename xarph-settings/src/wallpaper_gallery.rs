use gtk4::prelude::*;
use gtk4::{
    Box, Entry, FlowBox, FlowBoxChild, Label, Orientation, PolicyType, ScrolledWindow, ToggleButton,
};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static FAVORITES: Mutex<Option<Vec<String>>> = Mutex::new(None);

fn get_favorites_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("xarph");
    path.push("desktop");
    path.push("wallpaper_favorites.json");
    path
}

fn load_favorites() -> Vec<String> {
    let path = get_favorites_path();
    if let Ok(contents) = fs::read_to_string(&path) {
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_favorites(favorites: &[String]) {
    let path = get_favorites_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(contents) = serde_json::to_string_pretty(favorites) {
        let _ = fs::write(&path, contents);
    }
}

fn get_wallpaper_dirs() -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap_or_default();
    let data = dirs::data_dir().unwrap_or_default();
    let config = dirs::config_dir().unwrap_or_default();

    vec![
        PathBuf::from("/usr/share/backgrounds"),
        PathBuf::from("/usr/share/wallpapers"),
        home.join("Pictures/Wallpapers"),
        home.join("Pictures/Backgrounds"),
        data.join("wallpapers"),
        config.join("xarph/wallpapers"),
        home.join(".local/share/backgrounds"),
    ]
}

fn is_image_file(path: &PathBuf) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "webp" | "avif"
        ),
        None => false,
    }
}

fn collect_wallpapers(search: &str, show_favorites: bool) -> Vec<PathBuf> {
    let favorites = load_favorites();
    let mut results: Vec<PathBuf> = Vec::new();
    let search_lower = search.to_lowercase();

    for dir in get_wallpaper_dirs() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if is_image_file(&path) {
                    let name = entry.file_name().to_string_lossy().to_lowercase();

                    if show_favorites && !favorites.contains(&path.to_string_lossy().to_string()) {
                        continue;
                    }

                    if search_lower.is_empty() || name.contains(&search_lower) {
                        results.push(path);
                    }
                }
            }
        }
    }

    results.sort();
    results.dedup();
    results
}

pub fn build_wallpaper_gallery() -> Box {
    let vbox = Box::new(Orientation::Vertical, 6);
    vbox.set_margin_top(10);

    let header = Label::builder()
        .label("<b>Wallpaper Gallery</b>")
        .use_markup(true)
        .xalign(0.0)
        .build();
    vbox.append(&header);

    // Search bar
    let search_box = Box::new(Orientation::Horizontal, 4);
    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("Search wallpapers..."));
    search_entry.set_hexpand(true);
    search_box.append(&search_entry);

    let fav_toggle = ToggleButton::with_label("Favorites");
    fav_toggle.add_css_class("circular");
    search_box.append(&fav_toggle);
    vbox.append(&search_box);

    // FlowBox for thumbnails
    let flowbox = FlowBox::new();
    flowbox.set_max_children_per_line(4);
    flowbox.set_min_children_per_line(2);
    flowbox.set_selection_mode(gtk4::SelectionMode::None);
    flowbox.set_homogeneous(true);
    flowbox.set_column_spacing(8);
    flowbox.set_row_spacing(8);
    flowbox.set_activate_on_single_click(true);

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled.set_vexpand(true);
    scrolled.set_child(Some(&flowbox));
    vbox.append(&scrolled);

    // Clones for signal connections (before move into closure)
    let search_entry_clone = search_entry.clone();
    let fav_toggle_clone = fav_toggle.clone();

    let flowbox_clone = flowbox.clone();
    let load_wallpapers = move || {
        while let Some(child) = flowbox_clone.first_child() {
            flowbox_clone.remove(&child);
        }

        let search_text = search_entry_clone.text().to_string();
        let show_fav = fav_toggle_clone.is_active();
        let wallpapers = collect_wallpapers(&search_text, show_fav);

        for path in &wallpapers {
            let path_str = path.to_string_lossy().to_string();
            let child = FlowBoxChild::new();
            child.set_size_request(160, 120);

            let overlay = Box::new(Orientation::Vertical, 0);
            overlay.set_size_request(160, 120);

            if let Ok(texture) = gtk4::gdk::Texture::from_filename(path) {
                let picture = gtk4::Picture::for_paintable(&texture);
                picture.set_content_fit(gtk4::ContentFit::Cover);
                picture.set_size_request(160, 90);
                picture.set_hexpand(true);
                picture.set_halign(gtk4::Align::Fill);

                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let label = Label::builder()
                    .label(&name)
                    .max_width_chars(15)
                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                    .build();
                label.set_size_request(160, -1);

                overlay.append(&picture);
                overlay.append(&label);
            }

            child.set_child(Some(&overlay));

            let path_for_click = path_str.clone();
            child.connect_activate(move |_| {
                let mut config = xarph_sdk::config::XarphConfig::load();
                config.wallpaper_global = xarph_sdk::config::WallpaperConfig::Image {
                    path: path_for_click.clone(),
                    mode: xarph_sdk::config::WallpaperMode::Fill,
                };
                let _ = config.save();
            });

            flowbox_clone.append(&child);
        }

        flowbox_clone.show();
    };

    // Initial load
    load_wallpapers();

    // Update on search/filter changes
    search_entry.connect_changed({
        let loader = load_wallpapers.clone();
        move |_| loader()
    });
    fav_toggle.connect_toggled({
        let loader = load_wallpapers.clone();
        move |_| loader()
    });

    vbox
}
