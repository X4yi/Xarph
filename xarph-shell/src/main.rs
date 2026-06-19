mod backends;
mod bridges;
mod cli;
mod toggle_service;

use clap::Parser;
use cli::Cli;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl, QString};
use std::time::Duration;

fn main() {
    let _cli = Cli::parse();

    // Start toggle socket listener (receives Super key toggle from compositor)
    if let Err(e) = toggle_service::start_listener() {
        eprintln!("Warning: toggle socket listener failed to start: {e}");
    }

    // Start workspace event listener thread
    // Logs events for debugging; workspace data is updated via polling in QML Timers.
    std::thread::spawn(move || loop {
        match xarph_sdk::socket::Socket::connect() {
            Ok(socket) => {
                let mut event_reader = socket.read_events();
                loop {
                    match event_reader() {
                        Ok(event) => {
                            match &event {
                                xarph_sdk::Event::WorkspacesChanged { .. } => {
                                    // Workspace list changed — QML Timer will pick this up
                                }
                                xarph_sdk::Event::WindowFocusChanged { .. } => {
                                    // Focus changed — QML Timer will pick this up
                                }
                                _ => {}
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
            Err(_) => std::thread::sleep(Duration::from_secs(2)),
        }
    });

    // Initialize Qt application
    let mut app = QGuiApplication::new();
    app.pin_mut()
        .set_application_name(&QString::from("Xarph Shell"));
    app.pin_mut()
        .set_organization_name(&QString::from("Xarph"));

    // Create QML engine
    let mut engine = QQmlApplicationEngine::new();
    engine
        .pin_mut()
        .load(&QUrl::from(&QString::from("qrc:/qml/main.qml")));

    // Run event loop
    app.pin_mut().exec();
}
