use crate::api::middleware::auth::CurrentUser;
use crate::app_state::AppState;
use axum::{
    extract::{
        Extension, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(serde::Deserialize)]
pub struct TerminalQuery {
    pub node_id: Option<String>,
}

pub async fn node_terminal_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<TerminalQuery>,
    Extension(current_user): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| node_terminal_session(socket, query.node_id, current_user, state))
}

async fn node_terminal_session(
    socket: WebSocket,
    node_id: Option<String>,
    current_user: CurrentUser,
    _state: Arc<AppState>,
) {
    if let Some(ref nid) = node_id {
        tracing::info!(
            "User {} connecting to terminal for node {}",
            current_user.email,
            nid
        );
    } else {
        tracing::info!(
            "User {} connected to local host terminal",
            current_user.email
        );
    }

    // TODO: if node_id is NOT local manager, use SSH or proxy container
    terminal_session(socket, current_user).await;
}

async fn terminal_session(socket: WebSocket, current_user: CurrentUser) {
    tracing::info!("User {} connected to node terminal", current_user.email);

    let (mut ws_write, mut ws_read) = socket.split();

    // 1. Initialize PTY
    let pty_system = native_pty_system();
    let pair = match pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }) {
        Ok(p) => p,
        Err(e) => {
            let _ = ws_write
                .send(Message::Text(format!("Failed to open PTY: {}", e).into()))
                .await;
            return;
        }
    };

    // 2. Spawn shell
    let mut cmd = CommandBuilder::new("/bin/bash");
    if std::process::Command::new("/bin/bash")
        .arg("--version")
        .output()
        .is_err()
    {
        cmd = CommandBuilder::new("/bin/sh");
    }

    let mut _child = match pair.slave.spawn_command(cmd) {
        Ok(c) => c,
        Err(e) => {
            let _ = ws_write
                .send(Message::Text(
                    format!("Failed to spawn shell: {}", e).into(),
                ))
                .await;
            return;
        }
    };

    // 3. I/O Channels
    let mut reader: Box<dyn Read + Send> = match pair.master.try_clone_reader() {
        Ok(r) => r,
        Err(e) => {
            let _ = ws_write
                .send(Message::Text(format!("Failed to get reader: {}", e).into()))
                .await;
            return;
        }
    };
    let mut writer: Box<dyn Write + Send> = match pair.master.take_writer() {
        Ok(w) => w,
        Err(e) => {
            let _ = ws_write
                .send(Message::Text(format!("Failed to get writer: {}", e).into()))
                .await;
            return;
        }
    };

    let (pty_to_ws_tx, mut pty_to_ws_rx) = mpsc::channel::<Vec<u8>>(100);

    // Reader Task (Blocking -> Channel)
    tokio::task::spawn_blocking(move || {
        let mut buffer = [0u8; 1024];
        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 {
                break;
            }
            if pty_to_ws_tx.blocking_send(buffer[..n].to_vec()).is_err() {
                break;
            }
        }
    });

    // Processor Task
    let ws_to_pty = async move {
        while let Some(msg) = ws_read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if writer.write_all(text.as_bytes()).is_err() {
                        break;
                    }
                }
                Ok(Message::Binary(bin)) => {
                    if writer.write_all(&bin).is_err() {
                        break;
                    }
                }
                Ok(Message::Close(_)) => break,
                _ => {}
            }
        }
    };

    let pty_to_ws = async move {
        while let Some(data) = pty_to_ws_rx.recv().await {
            if ws_write.send(Message::Binary(data.into())).await.is_err() {
                break;
            }
        }
    };

    tokio::select! {
        _ = ws_to_pty => {},
        _ = pty_to_ws => {},
    }
}
