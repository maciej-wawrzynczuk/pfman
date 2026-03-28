mod adapters;
mod domain;
mod handlers;
mod state;

use std::net::SocketAddr;

use axum::{routing, Router};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let addr_str = std::env::var("PFMAN_LISTEN_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned());

    let addr: SocketAddr = match addr_str.parse() {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Invalid PFMAN_LISTEN_ADDR '{}': {}", addr_str, e);
            std::process::exit(1);
        }
    };

    tracing::info!("Listening on {}", addr);

    let state = state::initial_state();

    let app = Router::new()
        .route(
            "/transactions",
            routing::post(handlers::post_transactions::handler)
                .get(handlers::get_transactions::handler),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
