use askama_axum::{IntoResponse, Response};
use axum::http::StatusCode;
use sqlx::Error;

use crate::app::templates::{InternalServerErrorTemplate, NotFoundTemplate};

pub enum AppError {
    Database,
    NotFound
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Database => (StatusCode::INTERNAL_SERVER_ERROR, InternalServerErrorTemplate).into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, NotFoundTemplate).into_response()
        }
    }
}

impl From<Error> for AppError {
    fn from(_: Error) -> Self {
        Self::Database
    }
}
