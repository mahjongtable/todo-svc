use super::{
    model::{CreateTodo, Todo, UpdateTodo},
    repository::TodoRepository,
};
use sqlx::{Database, Error, MySql, Pool, pool::PoolOptions};

pub mod mysql {
    use super::*;

    pub struct MySqlTodoRepository {
        pool: Pool<MySql>,
    }

    impl MySqlTodoRepository {
        pub fn new(pool: Pool<MySql>) -> Self {
            Self { pool }
        }
    }

    #[async_trait::async_trait]
    impl TodoRepository for MySqlTodoRepository {
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
            let sql = "UPDATE `todos` SET `title` = $1, `description` = $2, `status` = $3 WHERE `id` = $4";

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
            let sql = "UPDATE `todos` SET `deleted_at` = NOW() WHERE `id` = $1 AND `deleted_at` IS NOT NULL";

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

        async fn get_todo_list(&self, page: u32, size: u32) -> Result<(Vec<Todo>, u64), Error> {
            let mut tx = self.pool.begin().await?;

            let sql = "SELECT COUNT(`id`) FROM `todos`";
            let total: i64 = sqlx::query_scalar(sql).fetch_one(&mut *tx).await?;

            let todos: Vec<Todo> = sqlx::query_as!(
                Todo,
                r#"
                SELECT
                    `id`,
                    `title`,
                    `description`,
                    `status`,
                    `created_at`,
                    `updated_at`,
                    `deleted_at`
                FROM `todos`
                WHERE
                    `deleted_at` IS NULL
                LIMIT ? OFFSET ?
                "#,
                size,
                (page - 1) * size
            )
            .fetch_all(&mut *tx)
            .await?;

            tx.commit().await?;

            Ok((todos, total as u64))
        }
    }
}

pub async fn connect<D: Database>() -> Result<Pool<D>, Error> {
    // let url = format!(
    //     "postgresql://{}:{}@{}:{}/{}",
    //     &cfg.username,
    //     &cfg.password,
    //     &cfg.host,
    //     &cfg.port.unwrap_or(3306),
    //     &cfg.database
    // );
    let url = "mysql://root:123456@localhost:3306/todo_service";

    let pool = PoolOptions::<D>::new()
        // .max_connections(cfg.max_connections.unwrap_or(5) as u32)
        // .max_connections(cfg.max_connections.unwrap_or(5) as u32)
        .connect(&url)
        .await?;

    Ok(pool)
}
