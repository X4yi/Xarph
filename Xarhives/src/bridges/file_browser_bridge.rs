/// File browser bridge: exposes file system operations to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, current_path)]
        #[qproperty(QString, parent_path)]
        #[qproperty(QString, selected_file)]
        #[qproperty(bool, can_go_up)]
        #[namespace = "xarph"]
        type FileBrowserBridge = super::FileBrowserBridgeRust;

        #[qinvokable]
        fn navigate(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        fn go_up(self: Pin<&mut Self>);

        #[qinvokable]
        fn go_home(self: Pin<&mut Self>);

        #[qinvokable]
        fn get_files(&self) -> QString;

        #[qinvokable]
        fn open_file(&self, path: &QString);

        #[qinvokable]
        fn create_folder(&self, name: &QString);

        #[qinvokable]
        fn delete_file(&self, path: &QString);

        #[qinvokable]
        fn rename_file(&self, old_path: &QString, new_name: &QString);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;
use std::path::PathBuf;

#[derive(Default)]
pub struct FileBrowserBridgeRust {
    current_path: QString,
    parent_path: QString,
    selected_file: QString,
    can_go_up: bool,
}

impl qobject::FileBrowserBridge {
    pub fn navigate(mut self: Pin<&mut Self>, path: &QString) {
        let path_str = path.to_string();
        let path_buf = PathBuf::from(&path_str);

        if path_buf.is_dir() {
            self.as_mut().set_current_path(path.clone());
            self.as_mut().set_can_go_up(path_buf.parent().is_some());

            if let Some(parent) = path_buf.parent() {
                self.as_mut()
                    .set_parent_path(QString::from(parent.to_str().unwrap_or("")));
            }
        }
    }

    pub fn go_up(mut self: Pin<&mut Self>) {
        let current = self.current_path().to_string();
        let path_buf = PathBuf::from(&current);

        if let Some(parent) = path_buf.parent() {
            let parent_str = parent.to_str().unwrap_or("");
            self.as_mut()
                .set_current_path(QString::from(parent_str));
            self.as_mut()
                .set_can_go_up(parent.parent().is_some());
        }
    }

    pub fn go_home(mut self: Pin<&mut Self>) {
        let home = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("~"))
            .to_str()
            .unwrap_or("")
            .to_string();
        self.as_mut().set_current_path(QString::from(&home));
        self.as_mut().set_can_go_up(true);
    }

    pub fn get_files(&self) -> QString {
        let current = self.current_path().to_string();
        let path = PathBuf::from(&current);

        let mut entries: Vec<String> = Vec::new();
        if let Ok(dir_entries) = std::fs::read_dir(&path) {
            for entry in dir_entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                let metadata = entry.metadata().ok();
                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                entries.push(format!(
                    "{}|{}|{}",
                    name,
                    if is_dir { "dir" } else { "file" },
                    size
                ));
            }
        }

        entries.sort();
        QString::from(&entries.join("\n"))
    }

    pub fn open_file(&self, path: &QString) {
        let path_str = path.to_string();
        std::thread::spawn(move || {
            let _ = std::process::Command::new("xdg-open")
                .arg(&path_str)
                .spawn();
        });
    }

    pub fn create_folder(&self, name: &QString) {
        let current = self.current_path().to_string();
        let name_str = name.to_string();
        let new_path = PathBuf::from(&current).join(&name_str);
        let _ = std::fs::create_dir(&new_path);
    }

    pub fn delete_file(&self, path: &QString) {
        let path_str = path.to_string();
        let path_buf = PathBuf::from(&path_str);
        if path_buf.is_dir() {
            let _ = std::fs::remove_dir_all(&path_buf);
        } else {
            let _ = std::fs::remove_file(&path_buf);
        }
    }

    pub fn rename_file(&self, old_path: &QString, new_name: &QString) {
        let old_str = old_path.to_string();
        let new_name_str = new_name.to_string();
        let old_path_buf = PathBuf::from(&old_str);

        if let Some(parent) = old_path_buf.parent() {
            let new_path = parent.join(&new_name_str);
            let _ = std::fs::rename(&old_path_buf, &new_path);
        }
    }
}
