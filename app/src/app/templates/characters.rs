use askama_axum::Template;

use crate::app::serializers::models::Character;

#[derive(Template)]
#[template(path = "characters/list.html")]
pub struct CharacterListTemplate {
    pub characters: Vec<Character>,
    pub path: String,
    pub total_pages: i64,
    pub page: i64,
    pub size: i64,
    pub name: String
}

#[derive(Template)]
#[template(path = "characters/detail.html")]
pub struct CharacterDetailTemplate {
    pub character: Character
}
