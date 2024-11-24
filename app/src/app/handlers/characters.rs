use askama_axum::{IntoResponse, Response};
use axum::extract::{OriginalUri, Path, Query, State};
use sqlx::{query_as, query_scalar, PgPool};

use crate::app::{
    serializers::{
        models::Character,
        query::{NameQuery, Paginator}
    },
    templates::{
        characters::{
            CharacterDetailTemplate,
            CharacterListTemplate
        },
        NotFoundTemplate
    }
};

pub async fn get_characters(
    OriginalUri(uri): OriginalUri,
    State(db): State<PgPool>,
    character: Option<Query<NameQuery>>,
    paginator: Option<Query<Paginator>>
) -> CharacterListTemplate {
    let Query(NameQuery { name }) = character.unwrap_or_default();
    let Query(Paginator { page, size }) = paginator.unwrap_or_default();
    let path = uri.path().to_string();
    let name_ilike = format!("%{}%", name);

    let characters =
        query_as::<_, Character>("select * from characters where name ilike $1 limit $2 offset $3")
        .bind(&name_ilike)
        .bind(size)
        .bind((page - 1) * size)
        .fetch_all(&db)
        .await
        .unwrap();

    let count: i64 =
        query_scalar("select count(*) from characters where name ilike $1")
        .bind(&name_ilike)
        .fetch_one(&db)
        .await
        .unwrap_or(0);

    let total_pages = ((count - 1) / size) + 1;

    CharacterListTemplate {
        characters,
        path,
        total_pages,
        page,
        size,
        name
    }
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
