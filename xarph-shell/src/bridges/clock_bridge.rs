/// Clock bridge: exposes time/date to QML via ClockService
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, time_text)]
        #[qproperty(QString, date_text)]
        #[qproperty(QString, clock_format)]
        #[qproperty(QString, date_format)]
        #[namespace = "xarph"]
        type ClockBridge = super::ClockBridgeRust;

        #[qinvokable]
        fn update_time(self: Pin<&mut Self>);

        #[qinvokable]
        fn set_format(self: Pin<&mut Self>, clock_fmt: &QString, date_fmt: &QString);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct ClockBridgeRust {
    time_text: QString,
    date_text: QString,
    clock_format: QString,
    date_format: QString,
}

impl qobject::ClockBridge {
    pub fn update_time(mut self: Pin<&mut Self>) {
        let clock_fmt = self.clock_format().to_string();
        let date_fmt = self.date_format().to_string();

        let time = xarph_sdk::clock_service::ClockService::format_time(
            if clock_fmt.is_empty() { "%H:%M" } else { &clock_fmt },
        );
        let date = xarph_sdk::clock_service::ClockService::format_time(
            if date_fmt.is_empty() { "%a %d %b" } else { &date_fmt },
        );

        self.as_mut().set_time_text(QString::from(&time));
        self.as_mut().set_date_text(QString::from(&date));
    }

    pub fn set_format(mut self: Pin<&mut Self>, clock_fmt: &QString, date_fmt: &QString) {
        self.as_mut().set_clock_format(clock_fmt.clone());
        self.as_mut().set_date_format(date_fmt.clone());
        self.update_time();
    }
}
