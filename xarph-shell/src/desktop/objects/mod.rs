pub mod application;
pub mod context_menu;
pub mod drag;
pub mod file_object;
pub mod folder;
pub mod persistence;
pub mod project;
pub mod shortcut;
pub mod widget_object;

use gtk4::Widget;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub use self::application::ApplicationObject;
pub use self::file_object::FileObject;
pub use self::folder::FolderObject;
pub use self::project::ProjectObject;
pub use self::shortcut::ShortcutObject;
pub use self::widget_object::WidgetObject;

// ── Object Data (persistence format) ──────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ObjectData {
    Folder {
        id: String,
        name: String,
        path: String,
        x: f64,
        y: f64,
    },
    File {
        id: String,
        name: String,
        path: String,
        x: f64,
        y: f64,
    },
    Application {
        id: String,
        name: String,
        desktop_file: String,
        x: f64,
        y: f64,
    },
    Project {
        id: String,
        name: String,
        path: String,
        x: f64,
        y: f64,
    },
    Shortcut {
        id: String,
        name: String,
        target: String,
        x: f64,
        y: f64,
    },
    Widget {
        id: String,
        name: String,
        wtype: String,
        x: f64,
        y: f64,
        width: i32,
        height: i32,
    },
}

impl ObjectData {
    pub fn id(&self) -> &str {
        match self {
            ObjectData::Folder { id, .. }
            | ObjectData::File { id, .. }
            | ObjectData::Application { id, .. }
            | ObjectData::Project { id, .. }
            | ObjectData::Shortcut { id, .. }
            | ObjectData::Widget { id, .. } => id,
        }
    }

    pub fn x(&self) -> f64 {
        match self {
            ObjectData::Folder { x, .. }
            | ObjectData::File { x, .. }
            | ObjectData::Application { x, .. }
            | ObjectData::Project { x, .. }
            | ObjectData::Shortcut { x, .. }
            | ObjectData::Widget { x, .. } => *x,
        }
    }

    pub fn y(&self) -> f64 {
        match self {
            ObjectData::Folder { y, .. }
            | ObjectData::File { y, .. }
            | ObjectData::Application { y, .. }
            | ObjectData::Project { y, .. }
            | ObjectData::Shortcut { y, .. }
            | ObjectData::Widget { y, .. } => *y,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ObjectData::Folder { name, .. }
            | ObjectData::File { name, .. }
            | ObjectData::Application { name, .. }
            | ObjectData::Project { name, .. }
            | ObjectData::Shortcut { name, .. }
            | ObjectData::Widget { name, .. } => name,
        }
    }
}

// ── DesktopObject Trait ──────────────────────────────────────────────

pub trait DesktopObject: Debug {
    fn id(&self) -> &str;
    fn object_type(&self) -> &str;
    fn data(&self) -> ObjectData;
    fn build(&self) -> Widget;
    fn set_position(&mut self, x: f64, y: f64);
}
