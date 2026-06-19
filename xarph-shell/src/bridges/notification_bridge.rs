/// Notification bridge: exposes notifications to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, title)]
        #[qproperty(QString, body)]
        #[qproperty(QString, icon)]
        #[qproperty(QString, urgency)]
        #[qproperty(QString, timestamp)]
        #[qproperty(QString, app_name)]
        #[namespace = "xarph"]
        type NotificationBridge = super::NotificationBridgeRust;

        #[qinvokable]
        fn send_notification(self: Pin<&mut Self>, title: &QString, body: &QString);

        #[qinvokable]
        fn send_notification_with_icon(
            self: Pin<&mut Self>,
            title: &QString,
            body: &QString,
            icon: &QString,
        );

        #[qinvokable]
        fn clear_notification(self: Pin<&mut Self>, id: &QString);

        #[qinvokable]
        fn clear_all_notifications(self: Pin<&mut Self>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct NotificationBridgeRust {
    title: QString,
    body: QString,
    icon: QString,
    urgency: QString,
    timestamp: QString,
    app_name: QString,
}

impl qobject::NotificationBridge {
    pub fn send_notification(self: Pin<&mut Self>, title: &QString, body: &QString) {
        let _ = xarph_sdk::notification_service::NotificationService::send_notification(
            &title.to_string(),
            &body.to_string(),
            xarph_sdk::notification_service::NotificationUrgency::Normal,
        );
    }

    pub fn send_notification_with_icon(
        self: Pin<&mut Self>,
        title: &QString,
        body: &QString,
        icon: &QString,
    ) {
        let _ = xarph_sdk::notification_service::NotificationService::send_with_icon(
            &title.to_string(),
            &body.to_string(),
            &icon.to_string(),
            xarph_sdk::notification_service::NotificationUrgency::Normal,
        );
    }

    pub fn clear_notification(self: Pin<&mut Self>, id: &QString) {
        let _ = xarph_sdk::notification_service::NotificationService::clear_notification(
            &id.to_string(),
        );
    }

    pub fn clear_all_notifications(self: Pin<&mut Self>) {
        let _ = xarph_sdk::notification_service::NotificationService::clear_all_notifications();
    }
}
