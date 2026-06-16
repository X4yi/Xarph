use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, CenterBox, Label, Orientation, Separator, Widget};
use xarph_sdk::config::{PanelConfig, PanelSection, PanelWidgetConfig, WidgetKind, XarphConfig};

use crate::widgets::ShellWidget;
use crate::widgets::clock::ClockWidget;
use crate::widgets::network::NetworkWidget;
use crate::widgets::start_button::StartButtonWidget;
use crate::widgets::system::SystemWidget;
use crate::widgets::tray::TrayWidget;
use crate::widgets::workspace::WorkspaceWidget;

pub fn build_panel(config: &XarphConfig, panel_config: &PanelConfig, no_tray: bool) -> CenterBox {
    let panel = CenterBox::new();
    panel.add_css_class("panel");

    let orientation = if panel_config.position.is_horizontal() {
        Orientation::Horizontal
    } else {
        Orientation::Vertical
    };

    let start = section_box(orientation, panel_config.spacing);
    let center = section_box(orientation, panel_config.spacing);
    let end = section_box(orientation, panel_config.spacing);

    append_widgets(
        config,
        panel_config,
        no_tray,
        &start,
        &center,
        &end,
        orientation,
    );

    panel.set_start_widget(Some(&start));
    panel.set_center_widget(Some(&center));
    panel.set_end_widget(Some(&end));
    panel
}

fn section_box(orientation: Orientation, spacing: i32) -> GtkBox {
    let section = GtkBox::new(orientation, spacing);
    section.add_css_class("panel-section");
    section
}

fn append_widgets(
    config: &XarphConfig,
    panel_config: &PanelConfig,
    no_tray: bool,
    start: &GtkBox,
    center: &GtkBox,
    end: &GtkBox,
    orientation: Orientation,
) {
    let mut first_by_section = [true, true, true];
    for widget_config in &panel_config.widgets {
        if !widget_config.visible {
            continue;
        }

        let Some(widget) = build_widget(config, widget_config, no_tray, orientation) else {
            continue;
        };

        let (target, first) = match widget_config.section {
            PanelSection::Start => (start, &mut first_by_section[0]),
            PanelSection::Center => (center, &mut first_by_section[1]),
            PanelSection::End => (end, &mut first_by_section[2]),
        };

        if !*first {
            target.append(&Separator::new(opposite_orientation(orientation)));
        }
        target.append(&widget);
        *first = false;
    }
}

fn build_widget(
    config: &XarphConfig,
    widget: &PanelWidgetConfig,
    no_tray: bool,
    orientation: Orientation,
) -> Option<Widget> {
    match widget.kind {
        WidgetKind::StartButton => Some(StartButtonWidget.build()),
        WidgetKind::Launcher => None, // Deprecated: use StartButton instead
        WidgetKind::Workspaces => Some(
            WorkspaceWidget {
                orientation,
                max_visible: widget.max_visible,
            }
            .build(),
        ),
        WidgetKind::Clock => Some(
            ClockWidget {
                clock_format: widget
                    .clock_format
                    .clone()
                    .or_else(|| config.clock_format.clone())
                    .unwrap_or_else(|| "%H:%M".to_string()),
                date_format: widget.date_format.clone(),
                update_interval: 1,
                orientation,
            }
            .build(),
        ),
        WidgetKind::Network => Some(
            NetworkWidget {
                interface: widget
                    .network_interface
                    .clone()
                    .or_else(|| config.network_interface.clone()),
                interval_ms: widget
                    .interval_ms
                    .or(config.system_refresh_ms)
                    .unwrap_or(2000),
            }
            .build(),
        ),
        WidgetKind::System => Some(
            SystemWidget {
                interval_ms: widget
                    .interval_ms
                    .or(config.system_refresh_ms)
                    .unwrap_or(2000),
                show_cpu: widget.show_cpu.or(config.show_cpu).unwrap_or(true),
                show_ram: widget.show_ram.or(config.show_ram).unwrap_or(true),
            }
            .build(),
        ),
        WidgetKind::Tray if no_tray => None,
        WidgetKind::Tray => Some(
            TrayWidget {
                icon_size: widget.icon_size.unwrap_or(22),
                orientation,
            }
            .build(),
        ),
        WidgetKind::ConfigButton => {
            Some(config_button(widget.label.as_deref().unwrap_or("Config")))
        }
    }
}

fn config_button(label: &str) -> Widget {
    let label = Label::new(Some(label));
    let button = Button::builder()
        .child(&label)
        .tooltip_text("Open Xarph settings")
        .css_classes(vec!["config-btn".to_string(), "flat".to_string()])
        .build();
    button.connect_clicked(|_| {
        if let Err(err) = std::process::Command::new("xarph-settings").spawn() {
            eprintln!("Failed to launch xarph-settings: {err}");
        }
    });
    button.upcast()
}

fn opposite_orientation(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::Horizontal => Orientation::Vertical,
        Orientation::Vertical => Orientation::Horizontal,
        _ => Orientation::Vertical,
    }
}
