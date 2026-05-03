use tokio::net::UnixStream;
use tokio::io::{AsyncBufReadExt, BufReader};
use serde_json::Value;
use crate::core::events::{Event, HyprlandEvent};
use crate::core::traits::EventChannel;

fn get_socket_path() -> Option<String> {
    let signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    Some(format!("/tmp/hypr/{}/.socket2.sock", signature))
}

async fn connect(socket_path: &str) -> Result<UnixStream, Box<dyn std::error::Error + Send + Sync>> {
    Ok(UnixStream::connect(socket_path).await?)
}

fn parse_hyprland_event(json: Value) -> Option<HyprlandEvent> {
    let event_type = json.get("event")?.as_str()?;
    let data = json.get("data")?;
    match event_type {
        "workspace" => {
            let id = data.get("id")?.as_u64()? as u32;
            let name = data.get("name")?.as_str()?.to_string();
            Some(HyprlandEvent::WorkspaceFocused { id, name })
        }
        "windowOpen" => {
            let id = data.get("id")?.as_u64()? as u32;
            let class = data.get("class")?.as_str()?.to_string();
            let workspace_id = data.get("workspace")?.get("id")?.as_u64()? as u32;
            Some(HyprlandEvent::WindowOpened { id, class, workspace_id })
        }
        "windowClose" => {
            let id = data.get("id")?.as_u64()? as u32;
            Some(HyprlandEvent::WindowClosed { id })
        }
        "monitorAdded" => {
            let name = data.get("name")?.as_str()?.to_string();
            let width = data.get("width")?.as_u64()? as u32;
            let height = data.get("height")?.as_u64()? as u32;
            Some(HyprlandEvent::MonitorAdded { name, width, height })
        }
        "monitorRemoved" => {
            let name = data.get("name")?.as_str()?.to_string();
            Some(HyprlandEvent::MonitorRemoved { name })
        }
        _ => None,
    }
}

async fn event_loop(stream: UnixStream, tx: EventChannel) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        tracing::debug!("Hyprland event: {}", line);
        if let Ok(json) = serde_json::from_str::<Value>(&line) {
            if let Some(event) = parse_hyprland_event(json) {
                let _ = tx.send(Event::Hyprland(event));
            }
        }
    }
    Ok(())
}

pub async fn run(tx: EventChannel) {
    let mut retry_delay = tokio::time::Duration::from_secs(1);
    loop {
        let socket_path = match get_socket_path() {
            Some(p) => p,
            None => {
                tracing::error!("HYPRLAND_INSTANCE_SIGNATURE not set, retrying...");
                tokio::time::sleep(retry_delay).await;
                retry_delay = std::cmp::min(retry_delay * 2, tokio::time::Duration::from_secs(30));
                continue;
            }
        };
        tracing::info!("Connecting to Hyprland socket: {}", socket_path);
        match connect(&socket_path).await {
            Ok(stream) => {
                retry_delay = tokio::time::Duration::from_secs(1);
                tracing::info!("Connected to Hyprland socket");
                if let Err(e) = event_loop(stream, tx.clone()).await {
                    tracing::error!("Hyprland event loop error: {}", e);
                }
            }
            Err(e) => {
                tracing::error!("Hyprland connection failed: {}", e);
            }
        }
        tokio::time::sleep(retry_delay).await;
        retry_delay = std::cmp::min(retry_delay * 2, tokio::time::Duration::from_secs(30));
    }
}
