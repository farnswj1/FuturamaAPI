mod app;

use app::get_router;
use axum::serve;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let address = "0.0.0.0:8000";
    let router = get_router().await.unwrap();
    let listener = TcpListener::bind(address).await.unwrap();

    info!("LISTENING on {address}");
    serve(listener, router).await.unwrap();
}
