/// Workspace bridge: exposes workspace state from IPC to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, workspace_id)]
        #[qproperty(i32, workspace_idx)]
        #[qproperty(QString, workspace_name)]
        #[qproperty(bool, is_focused)]
        #[qproperty(bool, is_active)]
        #[qproperty(bool, is_urgent)]
        #[qproperty(i32, active_window_id)]
        #[namespace = "xarph"]
        type WorkspaceBridge = super::WorkspaceBridgeRust;

        #[qinvokable]
        fn focus_workspace(self: Pin<&mut Self>, idx: i32);

        #[qinvokable]
        fn get_label(&self) -> QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, workspace_count)]
        #[qproperty(i32, focused_idx)]
        #[namespace = "xarph"]
        type WorkspaceModelBridge = super::WorkspaceModelBridgeRust;

        #[qinvokable]
        fn load_workspaces(self: Pin<&mut Self>);

        #[qinvokable]
        fn get_workspace_count(&self) -> i32;

        #[qinvokable]
        fn focus_workspace(self: Pin<&mut Self>, idx: i32);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct WorkspaceBridgeRust {
    workspace_id: i32,
    workspace_idx: i32,
    workspace_name: QString,
    is_focused: bool,
    is_active: bool,
    is_urgent: bool,
    active_window_id: i32,
}

#[derive(Default)]
pub struct WorkspaceModelBridgeRust {
    workspace_count: i32,
    focused_idx: i32,
}

impl qobject::WorkspaceBridge {
    pub fn focus_workspace(self: Pin<&mut Self>, idx: i32) {
        // Send IPC command to focus workspace
        std::thread::spawn(move || {
            if let Ok(mut socket) = xarph_sdk::socket::Socket::connect() {
                let request = xarph_sdk::Request::Action(xarph_sdk::Action::FocusWorkspace {
                    reference: xarph_sdk::WorkspaceReferenceArg::Index(idx as u8),
                });
                let _ = socket.send(request);
            }
        });
    }

    pub fn get_label(&self) -> QString {
        if self.workspace_name().is_empty() {
            QString::from(&format!("{}", self.workspace_idx() + 1))
        } else {
            self.workspace_name().clone()
        }
    }
}

impl qobject::WorkspaceModelBridge {
    pub fn load_workspaces(mut self: Pin<&mut Self>) {
        if let Ok(mut socket) = xarph_sdk::socket::Socket::connect() {
            if let Ok(Ok(xarph_sdk::Response::Workspaces(workspaces))) =
                socket.send(xarph_sdk::Request::Workspaces)
            {
                let count = workspaces.len() as i32;
                let focused = workspaces
                    .iter()
                    .position(|w| w.is_focused)
                    .map(|i| i as i32)
                    .unwrap_or(0);
                self.as_mut().set_workspace_count(count);
                self.as_mut().set_focused_idx(focused);
            }
        }
    }

    pub fn get_workspace_count(&self) -> i32 {
        *self.workspace_count()
    }

    pub fn focus_workspace(self: Pin<&mut Self>, idx: i32) {
        std::thread::spawn(move || {
            if let Ok(mut socket) = xarph_sdk::socket::Socket::connect() {
                let request = xarph_sdk::Request::Action(xarph_sdk::Action::FocusWorkspace {
                    reference: xarph_sdk::WorkspaceReferenceArg::Index(idx as u8),
                });
                let _ = socket.send(request);
            }
        });
    }
}
