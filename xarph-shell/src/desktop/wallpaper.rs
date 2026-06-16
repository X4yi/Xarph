use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Overlay, Picture, Widget, gdk};
use xarph_sdk::config::{WallpaperConfig, WallpaperMode};

#[derive(Clone)]
pub struct WallpaperRenderer {
    pub window: ApplicationWindow,
    overlay: Overlay,
    current_config: WallpaperConfig,
}

impl WallpaperRenderer {
    pub fn new(app: &gtk4::Application, use_layer_shell: bool) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Xarph Desktop")
            .build();

        let overlay = Overlay::new();
        overlay.set_hexpand(true);
        overlay.set_vexpand(true);

        let renderer = Self {
            window,
            overlay,
            current_config: WallpaperConfig::default(),
        };

        renderer.setup_window(use_layer_shell);
        renderer
    }

    fn setup_window(&self, use_layer_shell: bool) {
        if use_layer_shell {
            use gtk4_layer_shell::{Edge, Layer, LayerShell};

            self.window.init_layer_shell();
            self.window.set_namespace(Some("xarph-shell-background"));
            self.window.set_layer(Layer::Background);
            self.window.set_anchor(Edge::Top, true);
            self.window.set_anchor(Edge::Bottom, true);
            self.window.set_anchor(Edge::Left, true);
            self.window.set_anchor(Edge::Right, true);
        }

        let wallpaper_widget = Self::create_wallpaper_widget(&self.current_config);
        self.overlay.set_child(Some(&wallpaper_widget));
        self.window.set_child(Some(&self.overlay));
    }

    pub fn add_overlay_widget(&self, widget: &impl IsA<Widget>) {
        self.overlay.add_overlay(widget);
    }

    pub fn remove_overlay_widget(&self, widget: &impl IsA<Widget>) {
        self.overlay.remove_overlay(widget);
    }

    fn create_wallpaper_widget(config: &WallpaperConfig) -> Widget {
        match config {
            WallpaperConfig::Color { hex } => {
                let css = format!("widget {{ background-color: {}; }}", hex);
                let provider = gtk4::CssProvider::new();
                provider.load_from_data(&css);

                let widget = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                widget.set_hexpand(true);
                widget.set_vexpand(true);

                if let Some(display) = gdk::Display::default() {
                    gtk4::style_context_add_provider_for_display(
                        &display,
                        &provider,
                        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );
                }

                widget.upcast()
            }
            WallpaperConfig::Image { path, mode } => {
                if let Ok(texture) = gdk::Texture::from_filename(path) {
                    let picture = Picture::for_paintable(&texture);
                    picture.set_hexpand(true);
                    picture.set_vexpand(true);

                    match mode {
                        WallpaperMode::Fill => picture.set_content_fit(gtk4::ContentFit::Cover),
                        WallpaperMode::Fit => picture.set_content_fit(gtk4::ContentFit::Contain),
                        WallpaperMode::Stretch => picture.set_content_fit(gtk4::ContentFit::Fill),
                        WallpaperMode::Center => {
                            picture.set_content_fit(gtk4::ContentFit::ScaleDown)
                        }
                        WallpaperMode::Tile => picture.set_content_fit(gtk4::ContentFit::Fill),
                    }

                    picture.upcast()
                } else {
                    let widget = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                    widget.set_hexpand(true);
                    widget.set_vexpand(true);
                    widget.upcast()
                }
            }
        }
    }

    pub fn set_config(&mut self, config: WallpaperConfig) {
        self.current_config = config.clone();
        let wallpaper_widget = Self::create_wallpaper_widget(&config);
        self.overlay.set_child(Some(&wallpaper_widget));
    }

    pub fn present(&self) {
        self.window.present();
    }
}
