use sqlx::Error;

use super::model::{CreateTodo, Todo, UpdateTodo};

#[async_trait::async_trait]
pub trait TodoRepository {
    async fn create_todo(&self, create_todo: CreateTodo) -> Result<u64, Error>;
    async fn update_todo(&self, update_todo: UpdateTodo) -> Result<u64, Error>;
    async fn delete_todo(&self, id: u64) -> Result<Result<u64, &str>, Error>;
    async fn get_todo(&self, id: u64) -> Result<Option<Todo>, Error>;
}