//! Responsible ONLY for regime classification

use axum::{routing::post, Router};
use std::net::SocketAddr;

mod handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/regime", post(handler::detect_regime));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
