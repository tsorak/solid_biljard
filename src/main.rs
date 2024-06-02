use client_watcher::ClientWatcher;
use tokio::net::TcpListener;

use crate::state::State;

mod api;
mod build;
mod router;
mod state;

mod client_watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = State::new("foo");

    let tcp_listener = TcpListener::bind("localhost:3000")
        .await
        .expect("Failed to bind on port 3000");

    println!("Listening on http://localhost:3000");

    let client_watcher = ClientWatcher::new("./client", state.clone());

    tokio::select! {
        _ = async move { axum::serve(tcp_listener, router::router().with_state(state)).await } => {
            client_watcher.force_stop();
            client_watcher.wait_until_end().await;
        },
        _ = tokio::signal::ctrl_c() => {
            //ClientWatcher handles signals on its own, just wait for it to shutdown
            client_watcher.wait_until_end().await;
        }
    }

    Ok(())
}
