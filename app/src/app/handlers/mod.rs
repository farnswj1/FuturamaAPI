pub mod characters;
pub mod episodes;

use crate::app::templates::{IndexTemplate, NotFoundTemplate};

pub async fn index() -> IndexTemplate {
    IndexTemplate
}

pub async fn not_found() -> NotFoundTemplate {
    NotFoundTemplate
}
