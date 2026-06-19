use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .qrc("resources.qrc")
        .file("src/bridges/process_bridge.rs")
        .build();
}
