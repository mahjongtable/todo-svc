use chrono::NaiveDateTime;

use crate::data::model::Todo;

pub struct TodoDto {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: i8,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    // pub deleted_at: Option<NaiveDateTime>,
}

pub struct TodoListDto {
    pub todos: Vec<TodoDto>,
    pub total: u64,
    pub previous_page: u32,
    pub next_page: u32,
    pub total_pages: u32,
}

impl From<Todo> for TodoDto {
    fn from(
        Todo {
            id,
            title,
            description,
            status,
            created_at,
            updated_at,
            deleted_at: _,
        }: Todo,
    ) -> Self {
        Self {
            id,
            title,
            description,
            status,
            created_at,
            updated_at,
        }
    }
}
