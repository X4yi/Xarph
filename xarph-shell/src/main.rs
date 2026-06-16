pub mod app;
pub mod backend;
pub mod cli;
pub mod desktop;
pub mod panel;
pub mod recent_apps;
pub mod start_menu;
pub mod widgets;

use clap::Parser;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use backend::ShellBackend;
use backend::layer::LayerShellBackend;
use backend::windowed::WindowedBackend;
use cli::{Cli, Mode};
use xarph_sdk::config::{ConfigLoader, XarphConfig};

fn main() {
    // GTK registers its own flags on the process argv, so we parse our CLI
    // arguments *before* GTK ever sees them, then strip them so GTK doesn't
    // trip on unknown flags like --mode.
    let cli = Cli::parse();

    // Build the application with GIO_APPLICATION_HANDLES_OPEN so GTK never
    // tries to parse command-line arguments on its own.
    let application = Application::builder()
        .application_id("com.xarph.shell")
        .flags(gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    // Select backend based on parsed CLI mode
    let backend: Rc<dyn ShellBackend> = match cli.mode {
        Mode::Windowed => Rc::new(WindowedBackend),
        // Nested and Full both use LayerShell — the difference is in which
        // Wayland compositor the binary is launched inside.
        Mode::Nested => Rc::new(LayerShellBackend),
        Mode::Full => Rc::new(LayerShellBackend),
    };

    let config_path = cli.config.clone();
    let config = Rc::new(load_config(config_path.clone()));
    let watch_path = config_watch_path(config_path.as_deref());
    let no_tray = cli.no_tray;

    application.connect_activate(move |app| {
        // 1. Create and show the desktop surface (wallpaper background)
        let desktop = desktop::Desktop::new(app, backend.uses_layer_shell());
        desktop.present();

        let windows: Rc<RefCell<Vec<ApplicationWindow>>> = Rc::new(RefCell::new(Vec::new()));
        rebuild_panel_windows(app, backend.as_ref(), &config, no_tray, &windows);

        let (reload_tx, reload_rx) = async_channel::unbounded::<XarphConfig>();
        let reload_config_path = config_path.clone();
        let reload_tx_clone = reload_tx.clone();

        xarph_sdk::config::watch_config_in(watch_path.clone(), move |_| {
            let next = load_config(reload_config_path.clone());
            let _ = reload_tx_clone.send_blocking(next);
        });

        let app_ref = app.clone();
        let backend_ref = backend.clone();
        let windows_ref = Rc::clone(&windows);
        glib::spawn_future_local(async move {
            while let Ok(next_config) = reload_rx.recv().await {
                let next_config = Rc::new(next_config);
                rebuild_panel_windows(
                    &app_ref,
                    backend_ref.as_ref(),
                    &next_config,
                    no_tray,
                    &windows_ref,
                );
            }
        });
    });

    // Pass an empty argv slice so GTK doesn't process our custom flags
    application.run_with_args(&[] as &[&str]);
}

fn load_config(config_path: Option<std::path::PathBuf>) -> XarphConfig {
    if let Some(path) = config_path {
        match ConfigLoader::load_path(&path) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Failed to load config from {}: {err}", path.display());
                XarphConfig::load()
            }
        }
    } else {
        XarphConfig::load()
    }
}

fn config_watch_path(config_path: Option<&std::path::Path>) -> PathBuf {
    match config_path {
        Some(path) if path.is_dir() => path.to_path_buf(),
        Some(path) => path
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| path.to_path_buf()),
        None => xarph_sdk::config::config_dir(),
    }
}

fn rebuild_panel_windows(
    app: &Application,
    backend: &dyn ShellBackend,
    config: &XarphConfig,
    no_tray: bool,
    windows: &Rc<RefCell<Vec<ApplicationWindow>>>,
) {
    for window in windows.borrow_mut().drain(..) {
        window.close();
    }

    for panel_config in config.shell.panels.clone() {
        let window = ApplicationWindow::builder().application(app).build();
        backend.setup_window(&window, &panel_config);
        app::build_ui(&window, config, &panel_config, no_tray);
        window.present();
        windows.borrow_mut().push(window);
    }
}
