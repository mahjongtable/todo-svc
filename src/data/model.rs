use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: i8,
    // ! It will be filled automatically.
    pub created_at: Option<NaiveDateTime>,
    // ! It will be filled automatically.
    pub updated_at: Option<NaiveDateTime>,
    // ! It won't be filled automatically.
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<i8>,
}

impl CreateTodo {
    pub fn new(title: String, description: Option<String>, status: Option<i8>) -> Self {
        Self {
            title,
            description,
            status,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: i8,
}

impl UpdateTodo {
    pub fn new(id: u64, title: String, description: Option<String>, status: i8) -> Self {
        Self {
            id,
            title,
            description,
            status,
        }
    }
}
