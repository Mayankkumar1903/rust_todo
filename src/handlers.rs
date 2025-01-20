use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::{Collection, bson::doc};
use crate::models::Todo;

// Updated handler signatures to implement IntoResponse
pub async fn insert_todo(
    Extension(collection): Extension<Collection<Todo>>,
    Json(todo): Json<Todo>,
) -> impl IntoResponse {
    match collection.insert_one(todo, None).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn find_todo_by_title(
    Extension(collection): Extension<Collection<Todo>>,
    Path(title): Path<String>,
) -> impl IntoResponse {
    match collection.find_one(doc! { "title": title }, None).await {
        Ok(Some(todo)) => Json(todo).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_todo_by_title(
    Extension(collection): Extension<Collection<Todo>>,
    Path(title): Path<String>,
) -> impl IntoResponse {
    match collection.delete_one(doc! { "title": title }, None).await {
        Ok(result) if result.deleted_count == 1 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}