pub mod config;
mod handlers;
mod serializers;

use std::net::SocketAddr;

use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::http::HeaderValue;
use axum::Error;
use axum::{http::Method, routing::get, Router};
use axum_client_ip::SecureClientIpSource;
use config::Config;
use handlers::{
    characters::{get_character, get_characters},
    episodes::{get_episode, get_episodes},
    index,
    not_found
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub async fn get_router(config: &Config) -> Result<IntoMakeServiceWithConnectInfo<Router, SocketAddr>, Error> {
    let origins = config.cors_allowed_origins
        .split(" ")
        .map(|origin| origin.parse().unwrap())
        .collect::<Vec<HeaderValue>>();

    // Set up middleware layers
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET]);

    // Connect to DB
    let database = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Connected to database");

    // Enable serving static files
    let serve_dir = ServeDir::new("static");

    Ok(Router::new()
        .layer(SecureClientIpSource::RightmostXForwardedFor.into_extension())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .route("/", get(index))
        .route("/characters", get(get_characters))
        .route("/characters/:id", get(get_character))
        .route("/episodes", get(get_episodes))
        .route("/episodes/:id", get(get_episode))
        .nest_service("/static", serve_dir)
        .fallback(not_found)
        .with_state(database)
        .into_make_service_with_connect_info::<SocketAddr>())
}
