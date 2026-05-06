use crate::core::{
    types::Workspace,
    state::ArcStateStore,
    traits::StateStore,
};
use std::sync::Arc;
use zbus::SignalContext;
use serde::{Serialize, Deserialize};
use zbus::zvariant::Type;

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct WorkspaceDBus {
    pub id: u32,
    pub name: String,
    pub focused: bool,
    pub monitor: String,
}

impl From<Workspace> for WorkspaceDBus {
    fn from(ws: Workspace) -> Self {
        Self {
            id: ws.id,
            name: ws.name,
            focused: ws.focused,
            monitor: ws.monitor,
        }
    }
}

pub struct X4ShellV1 {
    workspace_state: Arc<ArcStateStore<Arc<Vec<Workspace>>>>,
}

impl X4ShellV1 {
    pub fn new(workspace_state: Arc<ArcStateStore<Arc<Vec<Workspace>>>>) -> Self {
        Self { workspace_state }
    }
}

#[zbus::interface(name = "org.x4yi.X4Shell.v1")]
impl X4ShellV1 {
    #[zbus(signal)]
    async fn workspace_changed(ctx: &SignalContext<'_>, workspace: WorkspaceDBus) -> zbus::Result<()>;

    async fn get_workspaces(&self) -> Vec<WorkspaceDBus> {
        self.workspace_state
            .get()
            .as_ref()
            .iter()
            .cloned()
            .map(WorkspaceDBus::from)
            .collect()
    }

    async fn switch_workspace(&self, id: u32) -> zbus::fdo::Result<()> {
        // Send command to Hyprland to switch workspace
        use std::process::Command;
        let output = Command::new("hyprctl")
            .arg("dispatch")
            .arg("workspace")
            .arg(id.to_string())
            .output();
        
        match output {
            Ok(out) if out.status.success() => {
                tracing::info!("Switched to workspace {}", id);
                Ok(())
            }
            Ok(out) => {
                let err = String::from_utf8_lossy(&out.stderr);
                tracing::error!("Failed to switch workspace: {}", err);
                Err(zbus::fdo::Error::Failed(err.to_string()))
            }
            Err(e) => {
                tracing::error!("Failed to execute hyprctl: {}", e);
                Err(zbus::fdo::Error::Failed(e.to_string()))
            }
        }
    }

    async fn ping(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

pub async fn emit_workspace_changed(
    ctx: &SignalContext<'_>,
    workspace: Workspace,
) -> zbus::Result<()> {
    X4ShellV1::workspace_changed(ctx, WorkspaceDBus::from(workspace)).await
}
