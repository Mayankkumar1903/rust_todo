use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        Todo { // MongoDB will generate this on insert
            title,
            description,
            completed: false,
            created_at: Utc::now(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
}