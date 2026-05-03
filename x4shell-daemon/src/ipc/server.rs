use crate::core::{
    state::ArcStateStore,
    traits::StateStore,
    types::Workspace,
};
use crate::ipc::interfaces::v1::{X4ShellV1, emit_workspace_changed};
use std::sync::Arc;
use tokio::task;
use zbus::SignalContext;

pub async fn start(
    workspace_state: Arc<ArcStateStore<Arc<Vec<Workspace>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::session().await?;
    let path = "/org/x4yi/X4Shell/v1";
    
    // Create interface implementation
    let iface = X4ShellV1::new(workspace_state.clone());
    
    // Register object
    connection.object_server().at(path, iface).await?;
    
    tracing::info!("D-Bus server started at {}", path);
    
    // Spawn signal emission task
    let conn_clone = connection.clone();
    let ws_state = workspace_state.clone();
    task::spawn(async move {
        let mut workspace_watch = ws_state.watch();
        while workspace_watch.changed().await.is_ok() {
            let workspaces = ws_state.get();
            if let Some(focused) = workspaces.as_ref().iter().find(|ws| ws.focused).cloned() {
                let ctx = SignalContext::new(&conn_clone, path).unwrap();
                if let Err(e) = emit_workspace_changed(&ctx, focused).await {
                    tracing::error!("Failed to emit workspace_changed signal: {}", e);
                }
            }
        }
    });
    
    // Keep server running until shutdown
    tokio::signal::ctrl_c().await?;
    tracing::info!("IPC server shutting down");
    Ok(())
}
