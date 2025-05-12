use sqlx::{Error, MySql, Pool};

use super::{
    model::{CreateTodo, Todo, UpdateTodo},
    repository::TodoRepository,
};

pub struct MySqlTodo {
    pool: Pool<MySql>,
}

impl MySqlTodo {
    pub fn new(pool: Pool<MySql>) -> Self {
        MySqlTodo { pool }
    }
}

#[async_trait::async_trait]
impl TodoRepository for MySqlTodo {
    async fn create_todo(&self, create_todo: CreateTodo) -> Result<u64, Error> {
        let sql = "INSERT INTO `todos` (`title`, `description`, `status`) VALUES (?, ?, ?)";
        let new_id = sqlx::query(sql)
            .bind(create_todo.title)
            .bind(create_todo.description)
            .bind(create_todo.status)
            .execute(&self.pool)
            .await?
            .last_insert_id();
        Ok(new_id)
    }

    async fn update_todo(&self, update_todo: UpdateTodo) -> Result<u64, Error> {
        let sql =
            "UPDATE `todos` SET `title` = $1, `description` = $2, `status` = $3 WHERE `id` = $4";

        sqlx::query(sql)
            .bind(update_todo.title)
            .bind(update_todo.description)
            .bind(update_todo.status)
            .bind(update_todo.id)
            .execute(&self.pool)
            .await?;

        Ok(update_todo.id)
    }

    async fn delete_todo(&self, id: u64) -> Result<Result<u64, &str>, Error> {
        let sql =
            "UPDATE `todos` SET `deleted_at` = NOW() WHERE `id` = $1 AND `deleted_at` IS NOT NULL";

        let rows_affected = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Ok(Err("The data with id 1 has been deleted."));
        }

        Ok(Ok(id))
    }

    async fn get_todo(&self, id: u64) -> Result<Option<Todo>, Error> {
        // let sql = "SELECT `id`, `title`, `description`, `status`, `created_at`, `updated_at`, `deleted_at` WHERE `id` = $1 AND `deleted_at` IS NULL";
        let todo = sqlx::query_as!(
            Todo,
            r#"SELECT `id`, `title`, `description`, `status`, `created_at`, `updated_at`, `deleted_at` FROM `todos` WHERE `id` = ? AND `deleted_at` IS NULL"#,
            id
        ).fetch_optional(&self.pool).await?;

        Ok(todo)
    }
}
