use crate::state::State;

mod build;
mod router;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let state = State::new("foo");

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind on port 3000");

    axum::serve(tcp_listener, router::router().with_state(state)).await?;

    Ok(())
}
