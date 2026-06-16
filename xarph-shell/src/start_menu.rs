use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, FlowBox, FlowBoxChild, Label, Orientation, Popover, ScrolledWindow,
    SearchEntry, Separator, Widget, gio,
};
use xarph_sdk::config::XarphConfig;

use crate::recent_apps;

#[derive(Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub icon: gio::Icon,
    pub app_info: gio::AppInfo,
}

pub struct StartMenu {
    popover: Popover,
    search_entry: SearchEntry,
    pinned_box: FlowBox,
    recent_box: FlowBox,
    all_grid: FlowBox,
    footer: GtkBox,
    all_apps: Vec<AppEntry>,
    pinned_apps: Vec<AppEntry>,
    recent_list: Vec<String>,
}

impl StartMenu {
    pub fn new(relative_to: &impl IsA<Widget>) -> Self {
        let all_apps = discover_apps();
        let config = XarphConfig::load();
        let pinned_ids: Vec<String> = config.pinned_apps.clone();
        let recent_list = recent_apps::load();

        let pinned_apps: Vec<AppEntry> = all_apps
            .iter()
            .filter(|a| pinned_ids.contains(&a.id))
            .cloned()
            .collect();

        // ── Popover container ─────────────────────────────────────
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_width_request(480);
        container.set_height_request(620);
        container.add_css_class("start-menu");

        // ── Search bar ────────────────────────────────────────────
        let search_box = GtkBox::new(Orientation::Horizontal, 6);
        search_box.set_margin_top(12);
        search_box.set_margin_start(12);
        search_box.set_margin_end(12);
        search_box.set_margin_bottom(4);

        let search_icon = Label::new(Some(""));
        search_icon.set_markup("<span size='13000'>\u{f002}</span>");
        search_box.append(&search_icon);

        let search = SearchEntry::new();
        search.set_placeholder_text(Some("Buscar aplicaciones\u{2026}"));
        search.set_hexpand(true);
        search.add_css_class("start-menu-search");
        search_box.append(&search);
        container.append(&search_box);

        // ── Scrolled content ──────────────────────────────────────
        let content = GtkBox::new(Orientation::Vertical, 2);
        content.add_css_class("start-menu-content");

        // Pinned section
        let pinned_box = FlowBox::new();
        pinned_box.set_max_children_per_line(6);
        pinned_box.set_selection_mode(gtk4::SelectionMode::None);
        pinned_box.set_hexpand(true);
        pinned_box.set_column_spacing(4);
        pinned_box.set_row_spacing(4);
        pinned_box.add_css_class("start-menu-pinned");
        if !pinned_apps.is_empty() {
            let pinned_header = Label::new(Some("Ancladas"));
            pinned_header.add_css_class("start-menu-section-header");
            content.append(&pinned_header);
        }
        content.append(&pinned_box);

        // Separator
        content.append(&Separator::new(Orientation::Horizontal));

        // Recent section
        let recent_box = FlowBox::new();
        recent_box.set_max_children_per_line(6);
        recent_box.set_selection_mode(gtk4::SelectionMode::None);
        recent_box.set_hexpand(true);
        recent_box.set_column_spacing(4);
        recent_box.set_row_spacing(4);
        recent_box.add_css_class("start-menu-recent");
        let recent_header = Label::new(Some("Recientes"));
        recent_header.add_css_class("start-menu-section-header");
        content.append(&recent_header);
        content.append(&recent_box);

        // Separator
        content.append(&Separator::new(Orientation::Horizontal));

        // All apps grid header
        let all_header = Label::new(Some("Todas las aplicaciones"));
        all_header.add_css_class("start-menu-section-header");
        content.append(&all_header);

        // All apps grid
        let all_grid = FlowBox::new();
        all_grid.set_max_children_per_line(6);
        all_grid.set_selection_mode(gtk4::SelectionMode::None);
        all_grid.set_hexpand(true);
        all_grid.set_column_spacing(4);
        all_grid.set_row_spacing(4);
        all_grid.add_css_class("start-menu-grid");
        content.append(&all_grid);

        let scrolled = ScrolledWindow::builder()
            .child(&content)
            .propagate_natural_width(true)
            .vexpand(true)
            .build();
        container.append(&scrolled);

        // ── Footer ────────────────────────────────────────────────
        let footer = GtkBox::new(Orientation::Horizontal, 4);
        footer.add_css_class("start-menu-footer");

        let settings_btn = Button::builder()
            .label("\u{f013} Configuraci\u{00f3}n")
            .css_classes(vec!["start-menu-footer-btn".to_string()])
            .build();
        let lock_btn = Button::builder()
            .label("\u{f023} Bloquear")
            .css_classes(vec!["start-menu-footer-btn".to_string()])
            .build();
        let quit_btn = Button::builder()
            .label("\u{f011} Salir")
            .css_classes(vec!["start-menu-footer-btn".to_string()])
            .build();

        settings_btn.connect_clicked(|_| {
            let _ = std::process::Command::new("xarph-settings").spawn();
        });
        lock_btn.connect_clicked(|_| {
            let _ = std::process::Command::new("xarph-lock").spawn();
        });
        quit_btn.connect_clicked(|_| {
            if let Ok(mut socket) = xarph_sdk::socket::Socket::connect() {
                let _ = socket.send(xarph_sdk::Request::Action(xarph_sdk::Action::Quit {
                    skip_confirmation: false,
                }));
            }
        });

        footer.append(&settings_btn);
        footer.append(&lock_btn);
        footer.append(&quit_btn);
        container.append(&footer);

        // ── Popover ───────────────────────────────────────────────
        let popover = Popover::builder()
            .child(&container)
            .autohide(true)
            .has_arrow(false)
            .position(gtk4::PositionType::Bottom)
            .build();
        popover.set_parent(relative_to);

        // ── Populate apps ─────────────────────────────────────────
        let menu = Self {
            popover,
            search_entry: search,
            pinned_box,
            recent_box,
            all_grid,
            footer,
            all_apps,
            pinned_apps,
            recent_list,
        };

        menu.populate_pinned();
        menu.populate_recent();
        menu.populate_all();
        menu.setup_search();
        menu
    }

    pub fn toggle(&self) {
        if self.popover.is_visible() {
            self.popover.popdown();
        } else {
            self.search_entry.set_text("");
            self.search_entry.grab_focus();
            self.popover.popup();
        }
    }

    fn populate_pinned(&self) {
        for entry in &self.pinned_apps {
            let btn = build_app_button(entry);
            self.pinned_box.append(&btn);
        }
    }

    fn populate_recent(&self) {
        let recent_ids = &self.recent_list;
        let mut is_empty = true;
        for entry in &self.all_apps {
            if recent_ids.contains(&entry.id) {
                let btn = build_app_button(entry);
                self.recent_box.append(&btn);
                is_empty = false;
            }
        }

        if is_empty {
            let empty = Label::new(Some("(ninguna)"));
            empty.add_css_class("start-menu-empty");
            self.recent_box.append(&empty);
        }
    }

    fn populate_all(&self) {
        for entry in &self.all_apps {
            let btn = build_app_button(entry);
            self.all_grid.append(&btn);
        }
    }

    fn setup_search(&self) {
        let grid = self.all_grid.clone();
        let search = self.search_entry.clone();
        search.connect_search_changed(move |entry| {
            let text = entry.text().to_lowercase();
            grid.set_filter_func(move |child| {
                if text.is_empty() {
                    return true;
                }
                if let Some(btn) = child.child().and_downcast_ref::<Button>() {
                    if let Some(app_box) = btn.child().and_downcast_ref::<GtkBox>() {
                        if let Some(label) = app_box.last_child().and_downcast_ref::<Label>() {
                            return label.label().to_lowercase().contains(&text);
                        }
                    }
                }
                true
            });
        });
    }
}

fn discover_apps() -> Vec<AppEntry> {
    let mut apps: Vec<AppEntry> = gio::AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show())
        .filter_map(|app| {
            let id = app.id()?.to_string();
            let name = app.display_name().to_string();
            let icon = app.icon()?.clone();
            Some(AppEntry {
                id,
                name,
                icon,
                app_info: app,
            })
        })
        .collect();
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn build_app_button(entry: &AppEntry) -> FlowBoxChild {
    let icon = gtk4::Image::from_gicon(&entry.icon);
    icon.set_pixel_size(40);
    icon.set_halign(gtk4::Align::Center);
    icon.add_css_class("start-menu-app-icon");

    let label = Label::new(Some(&entry.name));
    label.set_wrap(true);
    label.set_max_width_chars(12);
    label.set_xalign(0.5);
    label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    label.add_css_class("start-menu-app-label");

    let app_box = GtkBox::new(Orientation::Vertical, 4);
    app_box.set_halign(gtk4::Align::Center);
    app_box.set_valign(gtk4::Align::Center);
    app_box.append(&icon);
    app_box.append(&label);

    let btn = Button::builder()
        .child(&app_box)
        .css_classes(vec!["start-menu-app-btn".to_string()])
        .build();

    let app_info = entry.app_info.clone();
    let entry_id = entry.id.clone();
    btn.connect_clicked(move |_| {
        let _ = app_info.launch(&[], None::<&gio::AppLaunchContext>);
        recent_apps::track_launch(&entry_id);
    });

    let child = FlowBoxChild::new();
    child.set_child(Some(&btn));
    child
}
