use crate::core::{
    traits::{Service, EventChannel, StateStore},
    events::{Event, HyprlandEvent},
    types::Workspace,
    state::ArcStateStore,
};
use std::sync::Arc;
use async_trait::async_trait;

pub struct WorkspaceService {
    workspace_state: Arc<ArcStateStore<Arc<Vec<Workspace>>>>,
    event_tx: EventChannel,
}

impl WorkspaceService {
    pub fn new(
        workspace_state: Arc<ArcStateStore<Arc<Vec<Workspace>>>>,
        event_tx: EventChannel,
    ) -> Self {
        Self {
            workspace_state,
            event_tx,
        }
    }
}

#[async_trait]
impl Service for WorkspaceService {
    async fn start(&mut self, events: EventChannel) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_rx = events.subscribe();
        while let Ok(event) = event_rx.recv().await {
            match event {
                Event::Hyprland(HyprlandEvent::WorkspaceFocused { id, name }) => {
                    let mut workspaces = self.workspace_state.get().as_ref().clone();
                    
                    for ws in &mut workspaces {
                        ws.focused = ws.id == id;
                    }
                    
                    if !workspaces.iter().any(|ws| ws.id == id) {
                        workspaces.push(Workspace {
                            id,
                            name: name.clone(),
                            focused: true,
                            monitor: "Unknown".to_string(),
                            windows: Vec::new(),
                        });
                    }
                    
                    let new_state = Arc::new(workspaces);
                    self.workspace_state.set(new_state.clone());
                    
                    if let Some(ws) = new_state.iter().find(|ws| ws.focused) {
                        let _ = self.event_tx.send(Event::WorkspaceChanged(ws.clone()));
                    }
                }
                Event::Hyprland(HyprlandEvent::MonitorAdded { name, .. }) => {
                    let mut workspaces = self.workspace_state.get().as_ref().clone();
                    for ws in &mut workspaces {
                        if ws.monitor == "Unknown" {
                            ws.monitor = name.clone();
                        }
                    }
                    self.workspace_state.set(Arc::new(workspaces));
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Stopping workspace service");
        Ok(())
    }
}
