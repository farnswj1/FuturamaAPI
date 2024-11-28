pub mod characters;
pub mod episodes;

use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate;

#[derive(Template)]
#[template(path = "500.html")]
pub struct InternalServerErrorTemplate;
