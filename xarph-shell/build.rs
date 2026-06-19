use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qrc("resources.qrc")
        .file("src/bridges/desktop_bridge.rs")
        .file("src/bridges/panel_bridge.rs")
        .file("src/bridges/wallpaper_bridge.rs")
        .file("src/bridges/workspace_bridge.rs")
        .file("src/bridges/start_menu_bridge.rs")
        .file("src/bridges/tray_bridge.rs")
        .file("src/bridges/context_menu_bridge.rs")
        .file("src/bridges/clock_bridge.rs")
        .file("src/bridges/notification_bridge.rs")
        .build();
}
