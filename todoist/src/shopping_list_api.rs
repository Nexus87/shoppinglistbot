use async_trait::async_trait;
use crate::types::{
    requests::{
        Task,
        Command,
        WriteResource,
        GetProjectsRequest
    },
    primitives::Integer,
    todoist::GetProjectsResponse,
};
use serde::Serialize;

const URL: &str = "https://todoist.com/api/v7/sync";

type TodoistClient = reqwest::Client;

pub struct TodoistApi {
    token: String,
    client: TodoistClient,
}

impl TodoistApi {
    pub fn new(token: String) -> TodoistApi {
        TodoistApi {
            token,
            client: reqwest::Client::new(),
        }
    }

    async fn make_request<T> (&self, payload: &T) -> Result<reqwest::Response, reqwest::Error> where T: Serialize {
        let res = self.client
            .post(URL)
            .json(&payload)
            .send()
            .await?;
        
        println!("POST: {}", res.status());
        Ok(res)
    }
}

#[async_trait]
pub trait ShoppingListApi {
    async fn get_projects(&self) -> Result<GetProjectsResponse, reqwest::Error>;
    async fn add_tasks(&self, texts: &[&str], project_id: Integer) -> Result<(), reqwest::Error>;
    async fn add_task(&self, text: &str, project_id: Integer) -> Result<(), reqwest::Error> {
        let texts = [text];
        self.add_tasks(&texts, project_id).await
    }
}
#[async_trait]
impl ShoppingListApi for TodoistApi {
    async fn get_projects(&self) -> Result<GetProjectsResponse, reqwest::Error> {
        let json = GetProjectsRequest {
            token: self.token.clone(),
            sync_token: "*".to_string(),
            resource_types: "[\"projects\"]".to_string(),
        };
        let response:GetProjectsResponse  = self
            .make_request(&json)
            .await?
            .json()
            .await?;
        println!("{:#?}", response);
        Ok(response)
    }

    async fn add_tasks(&self, texts: &[&str], project_id: Integer) -> Result<(), reqwest::Error> {
        let commands: Vec<Command<Task>> = texts.iter()
            .map(|x| Task::new(x, project_id))
            .map(Command::new_add_task)
            .collect();

        let request = WriteResource::new(&commands, &self.token).unwrap();
        let _ = self.make_request(&request).await?;
        Ok(())
    }
}