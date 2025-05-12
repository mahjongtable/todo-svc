use todo_svc::pb::{
    CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse, GetTodoRequest,
    GetTodoResponse, UpdateTodoRequest, UpdateTodoResponse,
    todo_server::{Todo, TodoServer},
};
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic::transport::Server::builder()
        .add_service(TodoServer::new(TodoService {}))
        .serve("[::1]:50054".parse()?)
        .await?;

    Ok(())
}

pub struct TodoService {}

#[tonic::async_trait]
impl Todo for TodoService {
    async fn create_todo(
        &self,
        req: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        todo!()
    }

    async fn update_todo(
        &self,
        req: Request<UpdateTodoRequest>,
    ) -> Result<Response<UpdateTodoResponse>, Status> {
        todo!()
    }

    async fn delete_todo(
        &self,
        req: Request<DeleteTodoRequest>,
    ) -> Result<Response<DeleteTodoResponse>, Status> {
        todo!()
    }

    async fn get_todo(
        &self,
        req: Request<GetTodoRequest>,
    ) -> Result<Response<GetTodoResponse>, Status> {
        todo!()
    }
}
