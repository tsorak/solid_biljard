use axum::{serve::Serve, Router};
use tokio::net::TcpListener;

use crate::state::State;

mod api;
mod db;
mod ext;
mod router;
mod state;

#[cfg(debug_assertions)]
mod client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = State::new().await;

    let tcp_listener = TcpListener::bind("localhost:3000")
        .await
        .expect("Failed to bind on port 3000");

    println!("Listening on http://localhost:3000");

    let axum_handle = axum::serve(tcp_listener, router::router().with_state(state.clone()));

    #[cfg(debug_assertions)]
    dev(axum_handle, state).await?;

    #[cfg(not(debug_assertions))]
    axum_handle.await?;

    Ok(())
}

#[cfg(debug_assertions)]
async fn dev(axum_handle: Serve<Router, Router>, state: crate::State) -> anyhow::Result<()> {
    let mut client = client::Client::init("./client", state).await;
    let client_watcher = client
        .take_watcher()
        .expect("take_watcher should not be called more than once");

    client.ensure_node_modules().await?;
    client.build_client().await;

    tokio::select! {
        _ = async move { axum_handle.await } => {
            client_watcher.force_stop();
            client_watcher.wait_until_end().await;
        },
        _ = tokio::signal::ctrl_c() => {
            //ClientWatcher handles signals on its own, just wait for it to shutdown
            client_watcher.wait_until_end().await;
        }
    };

    Ok(())
}
