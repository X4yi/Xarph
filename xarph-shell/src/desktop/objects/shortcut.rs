use gtk4::prelude::*;
use gtk4::{Box, GestureClick, Image, Label, Widget, glib};

use super::{DesktopObject, ObjectData, context_menu};

#[derive(Debug, Clone)]
pub struct ShortcutObject {
    id: String,
    name: String,
    target: String,
    x: f64,
    y: f64,
}

impl ShortcutObject {
    pub fn new(name: String, target: String, x: f64, y: f64) -> Self {
        Self {
            id: glib::uuid_string_random().to_string(),
            name,
            target,
            x,
            y,
        }
    }

    pub fn from_data(data: &ObjectData) -> Option<Self> {
        match data {
            ObjectData::Shortcut {
                id,
                name,
                target,
                x,
                y,
            } => Some(Self {
                id: id.clone(),
                name: name.clone(),
                target: target.clone(),
                x: *x,
                y: *y,
            }),
            _ => None,
        }
    }
}

impl DesktopObject for ShortcutObject {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &str {
        "shortcut"
    }

    fn data(&self) -> ObjectData {
        ObjectData::Shortcut {
            id: self.id.clone(),
            name: self.name.clone(),
            target: self.target.clone(),
            x: self.x,
            y: self.y,
        }
    }

    fn build(&self) -> Widget {
        let container = Box::new(gtk4::Orientation::Vertical, 2);
        container.set_size_request(80, 80);
        container.add_css_class("desktop-object");

        let icon = Image::from_icon_name("emblem-symbolic-link-symbolic");
        icon.set_pixel_size(48);
        icon.set_hexpand(true);
        icon.set_halign(gtk4::Align::Center);

        let label = Label::new(Some(&self.name));
        label.set_wrap(true);
        label.set_max_width_chars(10);
        label.set_halign(gtk4::Align::Center);

        container.append(&icon);
        container.append(&label);

        let target = self.target.clone();
        let gesture = GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(move |_, n_press, _, _| {
            if n_press == 2 {
                let _ = std::process::Command::new("xdg-open").arg(&target).spawn();
            }
        });
        container.add_controller(gesture);

        context_menu::attach_context_menu(&container.clone().upcast::<Widget>(), self.data());

        container.upcast()
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}
