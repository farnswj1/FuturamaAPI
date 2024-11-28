pub mod characters;
pub mod episodes;
pub mod errors;

use axum::http::StatusCode;

use crate::app::templates::{IndexTemplate, NotFoundTemplate};

pub async fn index() -> IndexTemplate {
    IndexTemplate
}

pub async fn not_found() -> (StatusCode, NotFoundTemplate) {
    (StatusCode::NOT_FOUND, NotFoundTemplate)
}
