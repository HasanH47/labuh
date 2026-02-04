use crate::api::middleware::auth::CurrentUser;
use crate::error::Result;
use crate::usecase::stack::StackUsecase;
use axum::{
    extract::{
        Extension, Path, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::app_state::AppState;
use crate::error::AppError;

pub async fn container_exec_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let stack_usecase = state
        .stack_usecase
        .as_ref()
        .ok_or_else(|| AppError::Internal("Stack usecase missing".to_string()))?;

    // Verify ownership
    let _ = stack_usecase
        .verify_container_ownership(&id, &user.id)
        .await?;

    let stack_uc = stack_usecase.clone();
    Ok(ws.on_upgrade(move |socket| handle_container_socket(socket, stack_uc, id)))
}

async fn handle_container_socket(
    mut socket: WebSocket,
    stack_usecase: Arc<StackUsecase>,
    container_id: String,
) {
    use bollard::container::LogOutput;
    use bollard::exec::StartExecResults;
    use futures::StreamExt;

    // 1. Create exec instance (shell)
    let exec = match stack_usecase
        .runtime()
        .exec_command(&container_id, vec!["/bin/sh".to_string()])
        .await
    {
        Ok(e) => e,
        Err(e) => {
            let _ = socket
                .send(Message::Text(format!("Error: {}", e).into()))
                .await;
            return;
        }
    };

    // 2. Connect to the exec instance
    let attached = match stack_usecase.runtime().connect_exec(&exec.id).await {
        Ok(StartExecResults::Attached { output, input }) => (output, input),
        Ok(StartExecResults::Detached) => {
            let _ = socket
                .send(Message::Text("Error: Exec started in detached mode".into()))
                .await;
            return;
        }
        Err(e) => {
            let _ = socket
                .send(Message::Text(format!("Error: {}", e).into()))
                .await;
            return;
        }
    };

    let (docker_rx, mut docker_tx) = attached;
    let mut docker_rx = docker_rx;

    // 3. Bridge the WebSocket and Docker exec stream
    loop {
        tokio::select! {
            // From Docker to WebSocket
            Some(Ok(output)) = docker_rx.next() => {
                match output {
                    LogOutput::StdOut { message } | LogOutput::StdErr { message } | LogOutput::Console { message } => {
                        if socket.send(Message::Binary(message)).await.is_err() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            // From WebSocket to Docker
            Some(res) = socket.next() => {
                let msg = match res {
                    Ok(m) => m,
                    Err(_) => break,
                };

                match msg {
                    Message::Binary(bin) => {
                        if AsyncWriteExt::write_all(&mut docker_tx, &bin).await.is_err() {
                            break;
                        }
                    }
                    Message::Text(txt) => {
                        if AsyncWriteExt::write_all(&mut docker_tx, txt.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }
            else => break,
        }
    }
}
