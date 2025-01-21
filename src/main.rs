mod models; // Declare the models module
mod handlers; // Declare the handlers module
mod db; // Declare the db module for get_database
use axum::{extract::Extension, routing::{delete, get, post}, Router};
use mongodb::Collection;
use crate::handlers::{insert_todo,get_all_todos ,find_todo_by_id, delete_todo_by_id , update_todo_status_by_id};
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
        .route("/todo/:id", get(find_todo_by_id).layer(Extension(collection.clone())))
        //Get all todos
        .route("/alltodo",get(get_all_todos).layer(Extension(collection.clone())))
        // Delete Todo by title route
        .route("/todo/:id", delete(delete_todo_by_id).layer(Extension(collection.clone())))
        // Update todo by id 
        .route("/todo/:id",post(update_todo_status_by_id).layer(Extension(collection.clone())));


    // Start the server (using a specific address)
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
