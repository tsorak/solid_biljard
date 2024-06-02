use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
};

pub async fn ws(ws_upgrade: WebSocketUpgrade, state: State<crate::State>) -> Response {
    ws_upgrade.on_upgrade(|ws| async { handle_socket_connection(ws, state).await })
}

async fn handle_socket_connection(mut socket: WebSocket, mut state: State<crate::State>) {
    loop {
        tokio::select! {
            Some(msg) = socket.recv() => {
                let msg = if let Ok(msg) = msg {
                    dbg!(msg)
                } else {
                    // Client disconnected
                    return;
                };

                if socket.send(msg).await.is_err() {
                    // Client disconnected
                    return;
                }
            },
            Some(_refresh_request) = state.refresh_channel.recv() => {
                let _ = socket.send("refresh".into()).await;
            }
        }
    }
}
