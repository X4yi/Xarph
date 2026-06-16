use gtk4::prelude::*;
use gtk4::{Align, Box, Fixed, Label, Orientation, Separator, Widget, glib};

use super::{DesktopObject, ObjectData, context_menu};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    MiniClock,
    Calendar,
    SystemMonitor,
}

impl WidgetType {
    fn as_str(&self) -> &str {
        match self {
            WidgetType::MiniClock => "mini-clock",
            WidgetType::Calendar => "calendar",
            WidgetType::SystemMonitor => "system-monitor",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "mini-clock" => Some(WidgetType::MiniClock),
            "calendar" => Some(WidgetType::Calendar),
            "system-monitor" => Some(WidgetType::SystemMonitor),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WidgetObject {
    id: String,
    name: String,
    wtype: WidgetType,
    x: f64,
    y: f64,
    width: i32,
    height: i32,
}

impl WidgetObject {
    pub fn new(name: String, wtype: WidgetType, x: f64, y: f64) -> Self {
        let (width, height) = match wtype {
            WidgetType::MiniClock => (200, 100),
            WidgetType::Calendar => (280, 240),
            WidgetType::SystemMonitor => (240, 180),
        };
        Self {
            id: glib::uuid_string_random().to_string(),
            name,
            wtype,
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_data(data: &ObjectData) -> Option<Self> {
        match data {
            ObjectData::Widget {
                id,
                name,
                wtype,
                x,
                y,
                width,
                height,
            } => WidgetType::from_str(wtype).map(|wtype| Self {
                id: id.clone(),
                name: name.clone(),
                wtype,
                x: *x,
                y: *y,
                width: *width,
                height: *height,
            }),
            _ => None,
        }
    }

    pub fn widget_type(&self) -> WidgetType {
        self.wtype
    }

    fn build_content(&self) -> Widget {
        match self.wtype {
            WidgetType::MiniClock => build_mini_clock(),
            WidgetType::Calendar => build_calendar_widget(),
            WidgetType::SystemMonitor => build_system_monitor(),
        }
    }
}

impl DesktopObject for WidgetObject {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &str {
        "widget"
    }

    fn data(&self) -> ObjectData {
        ObjectData::Widget {
            id: self.id.clone(),
            name: self.name.clone(),
            wtype: self.wtype.as_str().to_string(),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    fn build(&self) -> Widget {
        let outer = Fixed::new();
        outer.set_size_request(self.width, self.height);

        let container = Box::new(Orientation::Vertical, 0);
        container.add_css_class("desktop-widget");
        container.set_size_request(self.width, self.height);

        // Title bar
        let titlebar = Box::new(Orientation::Horizontal, 4);
        titlebar.set_size_request(-1, 24);
        titlebar.add_css_class("widget-titlebar");

        let name_label = Label::builder()
            .label(&self.name)
            .xalign(0.0)
            .css_classes(["widget-title"])
            .build();
        titlebar.append(&name_label);

        container.append(&titlebar);
        container.append(&Separator::new(Orientation::Horizontal));

        // Content
        let content = self.build_content();
        content.set_vexpand(true);
        container.append(&content);

        outer.put(&container, 0.0, 0.0);

        let widget_upcast = outer.upcast::<Widget>();

        // Drag is handled by desktop::objects::drag::attach_drag_move in desktop/mod.rs

        // Context menu (right-click)
        context_menu::attach_context_menu(&container.clone().upcast::<Widget>(), self.data());

        widget_upcast
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

fn build_mini_clock() -> Widget {
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.set_halign(Align::Center);
    vbox.set_valign(Align::Center);

    let time_label = Label::builder().css_classes(["widget-clock-time"]).build();
    let date_label = Label::builder().css_classes(["widget-clock-date"]).build();

    let update = {
        let time = time_label.clone();
        let date = date_label.clone();
        move || {
            let now = glib::DateTime::now_local().unwrap();
            time.set_text(&now.format("%H:%M:%S").unwrap());
            date.set_text(&now.format("%A, %B %d").unwrap());
        }
    };
    update();

    glib::timeout_add_seconds_local(1, move || {
        update();
        glib::ControlFlow::Continue
    });

    vbox.append(&time_label);
    vbox.append(&date_label);
    vbox.upcast()
}

fn build_calendar_widget() -> Widget {
    let cal = gtk4::Calendar::new();
    cal.set_hexpand(true);
    cal.set_vexpand(true);
    cal.upcast()
}

fn build_system_monitor() -> Widget {
    let vbox = Box::new(Orientation::Vertical, 4);
    vbox.set_margin_top(8);
    vbox.set_margin_bottom(8);
    vbox.set_margin_start(8);
    vbox.set_margin_end(8);

    let cpu_label = Label::builder()
        .xalign(0.0)
        .css_classes(["monospace"])
        .build();
    let ram_label = Label::builder()
        .xalign(0.0)
        .css_classes(["monospace"])
        .build();

    let update = {
        let cpu = cpu_label.clone();
        let ram = ram_label.clone();
        move || {
            let cpu_usage = read_cpu_usage();
            let mem = read_mem_info();
            cpu.set_text(&format!("CPU: {:.0}%", cpu_usage));
            ram.set_text(&format!("RAM: {:.0}% / {:.0} MB", mem.1, mem.0));
        }
    };
    update();

    glib::timeout_add_seconds_local(3, move || {
        update();
        glib::ControlFlow::Continue
    });

    vbox.append(&cpu_label);
    vbox.append(&ram_label);
    vbox.upcast()
}

use std::sync::atomic::{AtomicU64, Ordering};

static PREV_IDLE: AtomicU64 = AtomicU64::new(0);
static PREV_TOTAL: AtomicU64 = AtomicU64::new(0);

fn read_cpu_usage() -> f64 {
    let Ok(contents) = std::fs::read_to_string("/proc/stat") else {
        return 0.0;
    };
    let line = contents.lines().next().unwrap_or("");
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return 0.0;
    }
    let user: u64 = parts[1].parse().unwrap_or(0);
    let nice: u64 = parts[2].parse().unwrap_or(0);
    let system: u64 = parts[3].parse().unwrap_or(0);
    let idle: u64 = parts[4].parse().unwrap_or(0);
    let total = user + nice + system + idle;

    let prev_idle = PREV_IDLE.load(Ordering::Relaxed);
    let prev_total = PREV_TOTAL.load(Ordering::Relaxed);
    PREV_IDLE.store(idle, Ordering::Relaxed);
    PREV_TOTAL.store(total, Ordering::Relaxed);

    let d_total = total.saturating_sub(prev_total);
    let d_idle = idle.saturating_sub(prev_idle);
    if d_total == 0 {
        return 0.0;
    }
    ((d_total - d_idle) as f64 / d_total as f64) * 100.0
}

fn read_mem_info() -> (f64, f64) {
    let Ok(contents) = std::fs::read_to_string("/proc/meminfo") else {
        return (0.0, 0.0);
    };
    let mut total = 0.0f64;
    let mut available = 0.0f64;
    for line in contents.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(val) = line.split_whitespace().nth(1) {
                total = val.parse::<f64>().unwrap_or(0.0);
            }
        }
        if line.starts_with("MemAvailable:") {
            if let Some(val) = line.split_whitespace().nth(1) {
                available = val.parse::<f64>().unwrap_or(0.0);
            }
        }
    }
    let used_mb = (total - available) / 1024.0;
    let used_pct = if total > 0.0 {
        (total - available) / total * 100.0
    } else {
        0.0
    };
    (used_mb, used_pct)
}
