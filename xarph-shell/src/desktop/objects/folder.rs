use gtk4::prelude::*;
use gtk4::{Box, GestureClick, Image, Label, Widget, glib};

use super::{DesktopObject, ObjectData, context_menu};

#[derive(Debug, Clone)]
pub struct FolderObject {
    id: String,
    name: String,
    path: String,
    x: f64,
    y: f64,
}

impl FolderObject {
    pub fn new(name: String, path: String, x: f64, y: f64) -> Self {
        Self {
            id: glib::uuid_string_random().to_string(),
            name,
            path,
            x,
            y,
        }
    }

    pub fn from_data(data: &ObjectData) -> Option<Self> {
        match data {
            ObjectData::Folder {
                id,
                name,
                path,
                x,
                y,
            } => Some(Self {
                id: id.clone(),
                name: name.clone(),
                path: path.clone(),
                x: *x,
                y: *y,
            }),
            _ => None,
        }
    }
}

impl DesktopObject for FolderObject {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &str {
        "folder"
    }

    fn data(&self) -> ObjectData {
        ObjectData::Folder {
            id: self.id.clone(),
            name: self.name.clone(),
            path: self.path.clone(),
            x: self.x,
            y: self.y,
        }
    }

    fn build(&self) -> Widget {
        let container = Box::new(gtk4::Orientation::Vertical, 2);
        container.set_size_request(80, 80);
        container.add_css_class("desktop-object");

        let icon = Image::from_icon_name("folder-symbolic");
        icon.set_pixel_size(48);
        icon.set_hexpand(true);
        icon.set_halign(gtk4::Align::Center);

        let label = Label::new(Some(&self.name));
        label.set_wrap(true);
        label.set_max_width_chars(10);
        label.set_halign(gtk4::Align::Center);

        container.append(&icon);
        container.append(&label);

        let path = self.path.clone();
        let gesture = GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(move |_, n_press, _, _| {
            if n_press == 2 {
                let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
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
