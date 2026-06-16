use super::ShellWidget;
use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation, Widget, glib};
use std::thread;
use xarph_sdk::socket::Socket;
use xarph_sdk::{Action, Event, Request, WorkspaceReferenceArg};

/// Messages sent from the background IPC thread to the GTK main thread.
#[derive(Debug, Clone)]
enum WsUpdate {
    /// Full workspace list snapshot, sorted by idx.
    WorkspacesChanged(Vec<WsEntry>),
    /// A workspace was activated on its output.
    WorkspaceActivated { id: u64, focused: bool },
    /// Urgency flag changed on a workspace.
    WorkspaceUrgencyChanged { id: u64, urgent: bool },
    /// The active window on a workspace changed.
    WorkspaceActiveWindowChanged {
        workspace_id: u64,
        active_window_id: Option<u64>,
    },
    /// IPC connection lost.
    Disconnected,
}

/// A single workspace entry for rendering.
#[derive(Debug, Clone)]
struct WsEntry {
    id: u64,
    idx: u8,
    name: String,
    is_focused: bool,
    is_active: bool,
    is_urgent: bool,
}

/// Render state held on the main thread.
#[derive(Debug, Clone, Default)]
struct WsRenderState {
    workspaces: Vec<WsEntry>,
    focused_id: Option<u64>,
}

/// Workspace indicator widget driven by IPC events (no polling).
pub struct WorkspaceWidget {
    pub orientation: Orientation,
    pub max_visible: Option<usize>,
}

impl Default for WorkspaceWidget {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            max_visible: None,
        }
    }
}

impl ShellWidget for WorkspaceWidget {
    fn build(&self) -> Widget {
        let container = Box::new(self.orientation, 4);
        container.add_css_class("workspace-widget");
        let max_visible = self.max_visible.unwrap_or(usize::MAX);

        let (tx, rx) = async_channel::unbounded::<WsUpdate>();

        // Background IPC thread: subscribes to event stream and forwards updates.
        spawn_ipc_thread(tx);

        // Main-thread receiver: async, event-driven, no polling.
        let container_ref = container.clone();
        glib::spawn_future_local(async move {
            let mut state = WsRenderState::default();

            while let Ok(msg) = rx.recv().await {
                let mut changed = false;

                match msg {
                    WsUpdate::WorkspacesChanged(entries) => {
                        state.workspaces = entries;
                        state.focused_id =
                            state.workspaces.iter().find(|w| w.is_focused).map(|w| w.id);
                        changed = true;
                    }
                    WsUpdate::WorkspaceActivated { id, focused } => {
                        if focused {
                            state.focused_id = Some(id);
                        }
                        for w in &mut state.workspaces {
                            if w.id == id {
                                w.is_active = true;
                                w.is_focused = focused;
                            } else if focused {
                                w.is_focused = false;
                            }
                        }
                        changed = true;
                    }
                    WsUpdate::WorkspaceUrgencyChanged { id, urgent } => {
                        if let Some(w) = state.workspaces.iter_mut().find(|w| w.id == id) {
                            w.is_urgent = urgent;
                        }
                        changed = true;
                    }
                    WsUpdate::WorkspaceActiveWindowChanged { .. } => {}
                    WsUpdate::Disconnected => {}
                }

                if changed {
                    rebuild_buttons(&container_ref, &state, max_visible);
                }
            }
        });

        container.upcast()
    }
}

/// Rebuild button children to reflect the current state.
fn rebuild_buttons(container: &Box, state: &WsRenderState, max_visible: usize) {
    // Remove existing children.
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }

    // Sort by idx as a safety measure (should already be sorted).
    let mut sorted = state.workspaces.clone();
    sorted.sort_by_key(|w| w.idx);

    for entry in sorted.iter().take(max_visible) {
        let is_focused = state.focused_id == Some(entry.id);

        let label_text = if entry.name.is_empty() {
            entry.idx.to_string()
        } else {
            entry.name.clone()
        };

        let mut classes = vec!["workspace-btn".to_string()];
        if is_focused {
            classes.push("active".to_string());
        } else if entry.is_urgent {
            classes.push("urgent".to_string());
        } else {
            classes.push("flat".to_string());
        }

        let btn = Button::builder()
            .label(&label_text)
            .css_classes(classes)
            .build();

        let ws_id = entry.id;
        btn.connect_clicked(move |_| {
            if let Ok(mut s) = Socket::connect() {
                let _ = s.send(Request::Action(Action::FocusWorkspace {
                    reference: WorkspaceReferenceArg::Id(ws_id),
                }));
            }
        });

        container.append(&btn);
    }
}

/// Background thread: connect to IPC, subscribe to event stream, forward updates.
fn spawn_ipc_thread(sender: async_channel::Sender<WsUpdate>) {
    thread::spawn(move || {
        loop {
            // Connect to compositor IPC.
            let mut socket = match Socket::connect() {
                Ok(s) => s,
                Err(_) => {
                    thread::sleep(std::time::Duration::from_secs(2));
                    continue;
                }
            };

            // Subscribe to the event stream.
            let read_event = match socket.send(Request::EventStream) {
                Ok(Ok(xarph_sdk::Response::Handled)) => socket.read_events(),
                _ => {
                    thread::sleep(std::time::Duration::from_secs(2));
                    continue;
                }
            };

            let mut read_event = read_event;

            // Read events until the connection drops.
            loop {
                match read_event() {
                    Ok(event) => {
                        let update = match &event {
                            Event::WorkspacesChanged { workspaces } => {
                                let mut entries: Vec<WsEntry> = workspaces
                                    .iter()
                                    .map(|w| WsEntry {
                                        id: w.id,
                                        idx: w.idx,
                                        name: w.name.clone().unwrap_or_else(|| w.idx.to_string()),
                                        is_focused: w.is_focused,
                                        is_active: w.is_active,
                                        is_urgent: w.is_urgent,
                                    })
                                    .collect();
                                entries.sort_by_key(|w| w.idx);
                                Some(WsUpdate::WorkspacesChanged(entries))
                            }
                            Event::WorkspaceActivated { id, focused } => {
                                Some(WsUpdate::WorkspaceActivated {
                                    id: *id,
                                    focused: *focused,
                                })
                            }
                            Event::WorkspaceUrgencyChanged { id, urgent } => {
                                Some(WsUpdate::WorkspaceUrgencyChanged {
                                    id: *id,
                                    urgent: *urgent,
                                })
                            }
                            Event::WorkspaceActiveWindowChanged {
                                workspace_id,
                                active_window_id,
                            } => Some(WsUpdate::WorkspaceActiveWindowChanged {
                                workspace_id: *workspace_id,
                                active_window_id: *active_window_id,
                            }),
                            _ => None,
                        };

                        if let Some(update) = update {
                            if sender.send_blocking(update).is_err() {
                                return;
                            }
                        }
                    }
                    Err(_) => {
                        let _ = sender.send_blocking(WsUpdate::Disconnected);
                        break;
                    }
                }
            }
        }
    });
}
