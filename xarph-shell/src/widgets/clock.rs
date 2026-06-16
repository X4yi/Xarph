use super::ShellWidget;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Widget, glib};

/// Configurable clock widget.
pub struct ClockWidget {
    /// strftime-compatible format string.
    pub clock_format: String,
    pub date_format: Option<String>,
    /// Update interval in seconds (default: 1)
    pub update_interval: u32,
    pub orientation: Orientation,
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self {
            clock_format: "%H:%M".to_string(),
            date_format: Some("%a %d %b".to_string()),
            update_interval: 1,
            orientation: Orientation::Horizontal,
        }
    }
}

impl ShellWidget for ClockWidget {
    fn build(&self) -> Widget {
        let clock_format = self.clock_format.clone();
        let date_format = self.date_format.clone().filter(|s| !s.is_empty());
        let update_interval = self.update_interval;

        let container = GtkBox::new(self.orientation, 4);
        container.add_css_class("clock-widget");

        let clock_label = Label::builder()
            .css_classes(vec!["clock-label".to_string()])
            .build();
        container.append(&clock_label);

        let date_label = date_format.as_ref().map(|_| {
            let label = Label::builder()
                .css_classes(vec!["date-label".to_string()])
                .build();
            container.append(&label);
            label
        });

        // Set initial time immediately to avoid flash
        tick(
            &clock_label,
            date_label.as_ref(),
            &clock_format,
            date_format.as_deref(),
        );

        let clock_label_clone = clock_label.clone();
        let date_label_clone = date_label.clone();
        glib::timeout_add_local(
            std::time::Duration::from_secs(update_interval as u64),
            move || {
                tick(
                    &clock_label_clone,
                    date_label_clone.as_ref(),
                    &clock_format,
                    date_format.as_deref(),
                );
                glib::ControlFlow::Continue
            },
        );

        container.upcast()
    }
}

fn tick(
    clock_label: &Label,
    date_label: Option<&Label>,
    clock_format: &str,
    date_format: Option<&str>,
) {
    let Ok(now) = glib::DateTime::now_local() else {
        clock_label.set_text("--:--");
        if let Some(label) = date_label {
            label.set_text("");
        }
        return;
    };

    if let Ok(text) = now.format(clock_format) {
        clock_label.set_text(&text);
    } else {
        clock_label.set_text("--:--");
    }

    if let (Some(label), Some(format)) = (date_label, date_format) {
        if let Ok(text) = now.format(format) {
            label.set_text(&text);
        }
    }
}
