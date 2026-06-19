use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qrc("resources.qrc")
        .file("src/bridges/service_bridge.rs")
        .build();
}
