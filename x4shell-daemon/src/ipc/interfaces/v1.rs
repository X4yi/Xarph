use crate::core::{
    types::Workspace,
    state::ArcStateStore,
    traits::StateStore,
};
use std::sync::Arc;
use zbus::{interface, SignalContext};
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
