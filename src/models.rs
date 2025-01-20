use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id")]  // Rename field to match MongoDB's _id
    pub id: Option<String>,   // MongoDB _id
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        Todo {
            id: None,  // MongoDB will generate this on insert
            title,
            description,
            completed: false,
            created_at: Utc::now(),
        }
    }
}
