mod collector;
mod near;
mod router;

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(router::metrics())
        .merge(router::probe());

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 8080))).await?;

    println!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
