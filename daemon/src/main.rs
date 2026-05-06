use crate::core::ArcStateStore;
use crate::core::traits::Service;
use crate::core::types::Workspace;
use std::sync::Arc;
use tokio::sync::broadcast;

mod core;
mod services;
mod adapters;
mod ipc;
mod config;
mod persistence;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting X4Shell daemon v{}", env!("CARGO_PKG_VERSION"));

    let (event_tx, _) = broadcast::channel::<core::Event>(1024);

    let (workspace_state, _) = ArcStateStore::new(Arc::new(Vec::<Workspace>::new()));
    let workspace_state = Arc::new(workspace_state);

    let hyprland_tx = event_tx.clone();
    tokio::spawn(async move {
        adapters::hyprland::run(hyprland_tx).await;
    });

    let mut workspace_svc = services::workspace::WorkspaceService::new(
        workspace_state.clone(),
        event_tx.clone(),
    );
    let svc_event_tx = event_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = workspace_svc.start(svc_event_tx).await {
            tracing::error!("Workspace service error: {}", e);
        }
    });

    let ipc_workspace_state = workspace_state.clone();
    tokio::spawn(async move {
        if let Err(e) = ipc::server::start(ipc_workspace_state).await {
            tracing::error!("IPC server error: {}", e);
        }
    });

    shutdown_signal().await;
    tracing::info!("Shutting down...");

    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    }
}
