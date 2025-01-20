use mongodb::{options::ClientOptions, Client};
use std::env;
use dotenvy::dotenv;

pub async fn get_database() -> mongodb::error::Result<Client> {
    // Load environment variables from .env file
    dotenv().ok();
    // Get the MongoDB URI from the .env file
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env file");

    // Parse the client options and create the client
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;

    // Display success message and return the client
    println!("Connected successfully to MongoDB 😀😀!");
    Ok(client)
}
