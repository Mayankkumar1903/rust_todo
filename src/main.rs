mod models; // Declare the models module
mod handlers; // Declare the handlers module
mod db; // Declare the db module for get_database
use axum::{Router, routing::{post, get, delete}, extract::Extension};
use mongodb::Collection;
use crate::handlers::{insert_todo, find_todo_by_title, delete_todo_by_title};
use crate::models::Todo;
use crate::db::get_database; // Import the get_database function

#[tokio::main]
async fn main() {
    // Initialize MongoDB client using get_database
    let client = get_database().await.unwrap();  // Retrieve the MongoDB client
    let collection: Collection<Todo> = client.database("test_db").collection("todos");

    let app = Router::new()
        // Insert Todo route
        .route("/todo", post(insert_todo).layer(Extension(collection.clone())))
        // View Todo by title route
        .route("/todo/:title", get(find_todo_by_title).layer(Extension(collection.clone())))
        // Delete Todo by title route
        .route("/todo/:title", delete(delete_todo_by_title).layer(Extension(collection.clone())));

    // Start the server (using a specific address)
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
