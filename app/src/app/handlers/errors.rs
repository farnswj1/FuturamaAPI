use askama_axum::{IntoResponse, Response};
use axum::http::StatusCode;
use sqlx::Error;

use crate::app::templates::InternalServerErrorTemplate;

pub struct AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, InternalServerErrorTemplate).into_response()
    }
}

impl From<Error> for AppError {
    fn from(_: Error) -> Self {
        Self
    }
}
