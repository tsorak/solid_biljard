// use std::time::Duration;

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
};

use crate::state::ClientChannelMessage;

pub async fn ws(ws_upgrade: WebSocketUpgrade, state: State<crate::State>) -> Response {
    ws_upgrade.on_upgrade(|ws| async {
        handle_socket_connection(ws, state).await;
    })
}

async fn handle_socket_connection(mut socket: WebSocket, mut state: State<crate::State>) {
    loop {
        tokio::select! {
            // _ = tokio::time::sleep(Duration::from_secs(5)) => {
            //     let _ = socket.close().await;
            //     println!("closed the socket lol");
            //     break;
            // }
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
            Some(m) = state.client_channel.recv() => {
                //putting "ClientChannelMessage::Refresh" in the above Some match does not work
                if matches!(m, ClientChannelMessage::Refresh) {
                    let _ = socket.send("refresh".into()).await;
                }
            }
        }
    }
}
