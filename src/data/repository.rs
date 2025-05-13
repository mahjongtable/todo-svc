use sqlx::Error;

use super::model::{CreateTodo, Todo, UpdateTodo};

#[async_trait::async_trait]
pub trait TodoRepository: Sync + Send + 'static {
    async fn create_todo(&self, create_todo: CreateTodo) -> Result<u64, Error>;
    async fn update_todo(&self, update_todo: UpdateTodo) -> Result<u64, Error>;
    async fn delete_todo(&self, id: u64) -> Result<Result<u64, &str>, Error>;
    async fn get_todo(&self, id: u64) -> Result<Option<Todo>, Error>;
    async fn get_todo_list(&self, page: u32, size: u32) -> Result<(Vec<Todo>, u64), Error>;
}