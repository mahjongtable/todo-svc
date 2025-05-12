use todo_svc::{
    data::{
        db::{self, mysql},
        model::{CreateTodo, UpdateTodo},
        repository::TodoRepository,
    },
    pb::{
        CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse,
        GetTodoRequest, GetTodoResponse, UpdateTodoRequest, UpdateTodoResponse,
        todo_server::{Todo, TodoServer},
    },
};
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = mysql::MySqlTodoRepository::new(db::connect().await?);

    tonic::transport::Server::builder()
        .add_service(TodoServer::new(TodoService { repo }))
        .serve("[::1]:50054".parse()?)
        .await?;

    Ok(())
}

pub struct TodoService<R: TodoRepository> {
    repo: R,
}

#[tonic::async_trait]
impl<R: TodoRepository> Todo for TodoService<R> {
    async fn create_todo(
        &self,
        req: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        let data = req.into_inner();

        let create_todo = CreateTodo::new(data.title, data.description, Some(0));

        let new_id = self
            .repo
            .create_todo(create_todo)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        let resp = CreateTodoResponse { id: new_id };

        Ok(Response::new(resp))
    }

    async fn update_todo(
        &self,
        req: Request<UpdateTodoRequest>,
    ) -> Result<Response<UpdateTodoResponse>, Status> {
        let req_inner = req.into_inner();

        let update_todo = UpdateTodo::new(
            req_inner.id,
            req_inner.title,
            req_inner.description,
            req_inner.status as i8,
        );

        self.repo
            .update_todo(update_todo)
            .await
            .map_err(|err| Status::internal(err.to_string()))
            .and_then(|_| Ok(Response::new(UpdateTodoResponse {})))
    }

    async fn delete_todo(
        &self,
        req: Request<DeleteTodoRequest>,
    ) -> Result<Response<DeleteTodoResponse>, Status> {
        self.repo
            .delete_todo(req.into_inner().id)
            .await
            .map_err(|db_err| Status::internal(db_err.to_string()))?
            .map_err(|err| Status::not_found(err.to_string()))
            .and_then(|_| Ok(Response::new(DeleteTodoResponse {})))
    }

    async fn get_todo(
        &self,
        req: Request<GetTodoRequest>,
    ) -> Result<Response<GetTodoResponse>, Status> {
        let id = req.get_ref().id;

        let todo = self
            .repo
            .get_todo(id)
            .await
            .map_err(|db_err| Status::internal(db_err.to_string()))?;

        match todo {
            Some(todo) => Ok(Response::new(GetTodoResponse {
                id: todo.id,
                title: todo.title,
                description: todo.description.unwrap_or_default(),
                status: todo.status as i32,
                // todo: convert to type 'Timestamp' type of proto3.
                created_at: None,
                // todo: convert to type 'Timestamp' type of proto3.
                updated_at: None,
            })),
            None => Err(Status::not_found(
                "The record may have been deleted or may not exist.",
            )),
        }
    }
}
