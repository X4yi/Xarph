mod bridges;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl, QString};

fn main() {
    let mut app = QGuiApplication::new();
    app.pin_mut()
        .set_application_name(&QString::from("Xarph Settings"));
    app.pin_mut()
        .set_organization_name(&QString::from("Xarph"));

    let mut engine = QQmlApplicationEngine::new();
    engine
        .pin_mut()
        .load(&QUrl::from(&QString::from("qrc:/qml/main.qml")));

    app.pin_mut().exec();
}
