mod config;
mod handlers;
mod log;
mod serializers;
mod templates;

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
use tower_http::trace::{DefaultOnFailure, TraceLayer};
use tracing::Level;
use log::{on_response, IPSpan};

pub async fn get_router() -> Result<IntoMakeServiceWithConnectInfo<Router, SocketAddr>, Error> {
    let config = envy::from_env::<Config>().unwrap();

    // Connect to DB
    let database = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Connected to database");

    // Create list of allowed origins
    let origins = config.cors_allowed_origins
        .split(" ")
        .map(|origin| origin.parse().unwrap())
        .collect::<Vec<HeaderValue>>();

    // Set up CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET]);

    // Set up logging middleware
    let logger = TraceLayer::new_for_http()
        .make_span_with(IPSpan)
        .on_response(on_response)
        .on_failure(DefaultOnFailure::new().level(Level::INFO));

    // Enable serving static files
    let serve_dir = ServeDir::new("static");

    Ok(Router::new()
        .route("/", get(index))
        .route("/characters", get(get_characters))
        .route("/characters/:id", get(get_character))
        .route("/episodes", get(get_episodes))
        .route("/episodes/:id", get(get_episode))
        .nest_service("/static", serve_dir)
        .fallback(not_found)
        .with_state(database)
        .layer(SecureClientIpSource::RightmostXForwardedFor.into_extension())
        .layer(logger)
        .layer(cors)
        .into_make_service_with_connect_info::<SocketAddr>())
}
