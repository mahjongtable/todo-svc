use todo_svc::{
    data::{
        db::{self, mysql},
        model::{CreateTodo, UpdateTodo},
        repository::TodoRepository,
    },
    dto::todo::{TodoDto, TodoListDto},
    pb::{
        CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse,
        GetTodoListRequest, GetTodoListResponse, GetTodoRequest, GetTodoResponse, TodoItem,
        UpdateTodoRequest, UpdateTodoResponse,
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
            Some(todo) => {
                let dto: TodoDto = todo.into();
                let resp: GetTodoResponseWrapper = dto.into();
                Ok(Response::new(resp.origin_response()))
            }
            None => Err(Status::not_found(
                "The record may have been deleted or may not exist.",
            )),
        }
    }

    async fn get_todo_list(
        &self,
        req: Request<GetTodoListRequest>,
    ) -> Result<Response<GetTodoListResponse>, Status> {
        let GetTodoListRequest { page, size } = req.into_inner();

        self.repo
            .get_todo_list(page as u32, size as u32)
            .await
            .and_then(|(todos, total)| {
                let total_pages = ((total) / (size as u64)) as u32;

                Ok(Response::new(GetTodoListResponse {
                    todos: todos
                        .into_iter()
                        .map(|todo| TodoItem {
                            id: todo.id,
                            title: todo.title,
                            description: todo.description.unwrap_or_default(),
                            status: todo.status as i32,
                            created_at: None,
                            updated_at: None,
                        })
                        .collect(),
                    previous_page: 0,
                    next_page: 0,
                    total: total as u64,
                    total_pages: total_pages,
                }))
            })
            .map_err(|err| Status::internal(err.to_string()))
    }
}

pub struct GetTodoResponseWrapper(pub GetTodoResponse);

impl From<TodoDto> for GetTodoResponseWrapper {
    fn from(value: TodoDto) -> Self {
        Self(GetTodoResponse {
            id: value.id,
            title: value.title,
            description: value.description.unwrap_or_default(),
            status: value.status as i32,
            created_at: None,
            updated_at: None,
        })
    }
}

impl GetTodoResponseWrapper {
    pub fn origin_response(self) -> GetTodoResponse {
        self.0
    }
}
