pub mod characters;
pub mod episodes;

use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> IndexTemplate {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate;

pub async fn not_found() -> NotFoundTemplate {
    NotFoundTemplate
}
