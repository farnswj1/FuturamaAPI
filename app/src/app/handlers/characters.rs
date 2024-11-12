use askama_axum::{IntoResponse, Response, Template};
use axum::extract::{Path, Query, State};
use sqlx::{query_as, PgPool};

use crate::app::serializers::{
    models::Character,
    query::{NameQuery, Paginator}
};

use super::NotFoundTemplate;

#[derive(Template)]
#[template(path = "characters/list.html")]
pub struct CharacterListTemplate {
    characters: Vec<Character>
}

pub async fn get_characters(
    State(db): State<PgPool>,
    character: Option<Query<NameQuery>>,
    paginator: Option<Query<Paginator>>
) -> CharacterListTemplate {
    let Query(NameQuery { name }) = character.unwrap_or_default();
    let Query(Paginator { page, size }) = paginator.unwrap_or_default();

    let characters =
        query_as::<_, Character>("select * from characters where name ilike $1 limit $2 offset $3")
        .bind(format!("%{}%", name))
        .bind(size)
        .bind((page - 1) * size)
        .fetch_all(&db)
        .await
        .unwrap();

    CharacterListTemplate { characters }
}

#[derive(Template)]
#[template(path = "characters/detail.html")]
pub struct CharacterDetailTemplate {
    character: Character
}

pub async fn get_character(
    State(db): State<PgPool>,
    Path(id): Path<String>
) -> Response {
    let query =
        query_as::<_, Character>("select * from characters where id::text = $1")
        .bind(id)
        .fetch_one(&db)
        .await;

    match query {
        Ok(character) => CharacterDetailTemplate { character }.into_response(),
        Err(_) => NotFoundTemplate.into_response()
    }
}
