use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "ok" }));

    let listener = TcpListener::bind("0.0.0.0:8083").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
