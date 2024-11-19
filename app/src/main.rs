mod app;

use app::get_router;
use axum::serve;
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let router = get_router().await.unwrap();
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("LISTENING on 0.0.0.0:8000");
    serve(listener, router).await.unwrap();
}
