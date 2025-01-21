use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use futures_util::StreamExt as _;
use mongodb::{bson::{doc, oid::ObjectId}, Collection};
use crate::models::{NewTodo, Todo};



pub async fn insert_todo(
    Extension(collection): Extension<Collection<Todo>>,
    Json(new_todo): Json<NewTodo>,
) -> impl IntoResponse {
    // Use the `Todo` model from `models.rs`
    let todo = Todo::new(new_todo.title, new_todo.description);

    match collection.insert_one(todo, None).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}


pub async fn get_all_todos(
    Extension(collection): Extension<Collection<Todo>>,
) -> impl IntoResponse {
    // Fetch all Todo documents from the collection
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut todos = Vec::new();

            // Iterate through the cursor and collect todos
            while let Some(todo_result) = cursor.next().await {
                match todo_result {
                    Ok(todo) => todos.push(todo),
                    Err(_) => {
                        // If an error occurs during iteration, return an internal error
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
            }

            // Return the list of todos as JSON
            Json(todos).into_response()
        }
        Err(_) => {
            // If there was an issue with the find operation, return an internal error
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


pub async fn find_todo_by_id(
    Extension(collection): Extension<Collection<Todo>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match ObjectId::parse_str(&id) {
        Ok(object_id) => {
            match collection.find_one(doc! { "_id": object_id }, None).await {
                Ok(Some(todo)) => Json(todo).into_response(),
                Ok(None) => StatusCode::NOT_FOUND.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(), // Invalid ObjectId format
    }
}

pub async fn delete_todo_by_id(
    Extension(collection): Extension<Collection<Todo>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match ObjectId::parse_str(&id) {
        Ok(object_id) => {
            match collection.delete_one(doc! { "_id": object_id }, None).await {
                Ok(result) if result.deleted_count == 1 => StatusCode::NO_CONTENT.into_response(),
                Ok(_) => StatusCode::NOT_FOUND.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(), // Invalid ObjectId format
    }
}

pub async fn update_todo_status_by_id(
    Extension(collection): Extension<Collection<Todo>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match ObjectId::parse_str(&id) {
        Ok(object_id) => {
            // Find the Todo by _id first
            match collection.find_one(doc! { "_id": object_id }, None).await {
                Ok(Some(mut todo)) => {
                    // If found, update the "completed" status to true
                    todo.completed = true;

                    // Perform the update operation
                    match collection.update_one(
                        doc! { "_id": object_id },
                        doc! { "$set": { "completed": true } },
                        None
                    ).await {
                        Ok(result) if result.modified_count == 1 => StatusCode::OK.into_response(),
                        Ok(_) => StatusCode::NOT_FOUND.into_response(),
                        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                    }
                }
                Ok(None) => StatusCode::NOT_FOUND.into_response(), // If Todo not found
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(), // Invalid ObjectId format
    }
}