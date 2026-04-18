use crate::requests::{Command, CommandRequest, SyncResponse, TaskId};
use crate::reqwest::Client;
use crate::todoist::{GetTasksResponse, Item};
use crate::types::{requests::Task, todoist::GetProjectsResponse};
use reqwest::Error;
use serde::Serialize;
use serde::de::DeserializeOwned;

const SYNC_URL: &str = "https://api.todoist.com/api/v1/sync";
const GET_PROJECTS: &str = "https://api.todoist.com/api/v1/projects";
const GET_TASKS_URL: &str = "https://api.todoist.com/api/v1/tasks";

pub struct TodoistApi {
    token: String,
    client: Client,
}

impl TodoistApi {
    pub fn new(token: String) -> TodoistApi {
        TodoistApi {
            token,
            client: Client::builder()
                .connection_verbose(true)
                .build()
                .unwrap(),
        }
    }

    async fn post_json_request<T: Serialize, TResult: DeserializeOwned>(
        &self,
        url: &str,
        payload: T,
        token: &String,
    ) -> Result<TResult, Error> {
        let res = self
            .client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
            .json(&payload)
            .send()
            .await?
            .json::<TResult>()
            .await?;
        Ok(res)
    }
    async fn send_command<TResult: DeserializeOwned>(
        &self,
        url: &str,
        payload: &CommandRequest,
        token: &String,
    ) -> Result<TResult, Error> {
        let res = self
            .client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
            .form(&payload)
            .send()
            .await?
            .json::<TResult>()
            .await?;
        Ok(res)
    }

    async fn get_request<TResult: DeserializeOwned + std::fmt::Debug>(
        &self,
        url: &str,
        token: &String,
    ) -> Result<TResult, Error> {
        let response = self
            .client
            .get(url)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?
            .json::<TResult>()
            .await?;
        Ok(response)
    }

    pub async fn get_projects(&self) -> Result<GetProjectsResponse, Error> {
        let response: GetProjectsResponse = self.get_request(GET_PROJECTS, &self.token).await?;
        Ok(response)
    }

    pub async fn add_tasks(&self, texts: &[&str], project_id: &str) -> Result<(), Error> {
        let commands: Vec<Command<Task>> = texts
            .iter()
            .map(|x| Task::new(x, project_id))
            .map(Command::new_add_task)
            .collect();
        let payload = CommandRequest::new(&commands).unwrap();
        let _: SyncResponse = self.send_command(SYNC_URL, &payload, &self.token).await?;
        Ok(())
    }

    pub async fn delete_tasks(&self, ids: &[String]) -> Result<(), Error> {
        let commands: Vec<Command<TaskId>> = ids
            .iter()
            .map(|x| TaskId::new(x))
            .map(Command::delete_task)
            .collect();

        let payload = CommandRequest::new(&commands).unwrap();
        let _: SyncResponse = self
            .post_json_request(SYNC_URL, &payload, &self.token)
            .await?;
        Ok(())
    }

    pub async fn get_tasks(&self, project_id: &str) -> Result<Vec<Item>, Error> {
        let url = format!("{}?project_id={}", GET_TASKS_URL, project_id);
        let response: GetTasksResponse = self.get_request(&url, &self.token).await?;
        Ok(response.results)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_projects_works() {
        let token = std::env::var("TOKEN").unwrap();
        let todolist_api = TodoistApi::new(token);
        let result = todolist_api.get_projects().await.unwrap();
        assert!(result.results.iter().any(|x| x.name == "Willkommen 👋"));
    }

    #[tokio::test]
    async fn add_and_remove_task_works() {
        let token = std::env::var("TOKEN").unwrap();
        let todolist_api = TodoistApi::new(token);
        let result = todolist_api.get_projects().await.unwrap();
        let project = result
            .results
            .iter()
            .find(|x| x.name == "Willkommen 👋")
            .unwrap();
        assert!(
            todolist_api
                .add_tasks(&*vec!["Test Task 1", "Test Task 2"], &project.id)
                .await
                .is_ok()
        );

        let tasks = todolist_api.get_tasks(&project.id).await.unwrap();
        let test_task1 = tasks.iter()
            .filter(|x| x.content == "Test Task 1")
            .map(|x| x.id.clone())
            .collect::<Vec<_>>();
        let test_task2 = tasks.iter().filter(|x| x.content == "Test Task 2")
            .map(|x| x.id.clone())
            .collect::<Vec<_>>();

        assert!(test_task1.len() > 0);
        assert!(test_task2.len() > 0);
        
        assert!(
            todolist_api
                .delete_tasks(
                    &[test_task1, test_task2].concat()
                )
                .await
                .is_ok()
        );
    }
}
