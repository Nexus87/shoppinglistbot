use crate::types::{
    primitives::Integer,
    requests::{Command, Task, WriteResource},
    todoist::GetProjectsResponse,
};
use crate::{requests::GetProjectsRequest, todoist::Item};
use hyper::{body::Buf, client::HttpConnector, header::HeaderValue, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use std::io::Read;
macro_rules! URL_BASE {
    () => {
        "https://api.todoist.com/sync/v8"
    };
}
const SYNC_URL: &str = concat!(URL_BASE!(), "/sync");
const GET_TASKS_URL: &str = "https://api.todoist.com/rest/v1/tasks";

type TodoistClient = Client<HttpsConnector<HttpConnector>>;

pub struct TodoistApi {
    token: String,
    client: TodoistClient,
}

impl TodoistApi {
    pub fn new(token: String) -> TodoistApi {
        let https = HttpsConnector::new();
        TodoistApi {
            token,
            client: Client::builder().build(https),
        }
    }

    async fn post_request(
        &self,
        url: &str,
        payload: String,
    ) -> Result<impl hyper::body::Buf, hyper::Error> {
        println!("{}", url);
        let uri: hyper::Uri = url.parse().unwrap();
        let mut req = Request::new(Body::from(payload));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let res = self.client.request(req).await?;
        // println!("POST: {}", res.status());

        Ok(hyper::body::aggregate(res).await?)
    }

    async fn get_request(
        &self,
        url: &str,
        token: String,
    ) -> Result<impl hyper::body::Buf, hyper::Error> {
        println!("{}", url);
        let uri: hyper::Uri = url.parse().unwrap();
        let mut req = Request::new(Body::empty());
        let token = format!("Bearer {}", token);
        *req.method_mut() = Method::GET;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::AUTHORIZATION,
            HeaderValue::from_str(token.as_str()).unwrap(),
        );
        let res = self.client.request(req).await?;

        Ok(hyper::body::aggregate(res).await?)
    }

    pub async fn get_projects(&self) -> Result<GetProjectsResponse, hyper::Error> {
        let json = GetProjectsRequest {
            token: self.token.clone(),
            sync_token: "*".to_string(),
            resource_types: "[\"projects\"]".to_string(),
        };
        let payload = serde_json::to_string(&json).unwrap();
        let body = self.post_request(SYNC_URL, payload).await?;
        let response = serde_json::from_reader(body.reader()).unwrap();
        println!("{:#?}", response);
        Ok(response)
    }

    pub async fn add_tasks(&self, texts: &[&str], project_id: Integer) -> Result<(), hyper::Error> {
        let commands: Vec<Command<Task>> = texts
            .iter()
            .map(|x| Task::new(x, project_id))
            .map(Command::new_add_task)
            .collect();

        let request = WriteResource::new(&commands, &self.token).unwrap();
        let payload = serde_json::to_string(&request).unwrap();
        self.post_request(SYNC_URL, payload).await?;
        Ok(())
    }

    pub async fn get_tasks(&self, project_id: Integer) -> Result<Vec<Item>, hyper::Error> {
        let url = format!("{}?project_id={}", GET_TASKS_URL, project_id);
        let response = self.get_request(&url, self.token.clone()).await?;
        let mut response_string = String::new();
        response
            .reader()
            .read_to_string(&mut response_string)
            .unwrap();

        println!("{}", response_string);
        let response: Result<Vec<Item>, _> = serde_json::from_str(response_string.as_str());
        // let response: Result<GetDataResponse, _> = serde_json::from_reader(response.reader());
        match response {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{}", e);
                panic!(e)
            }
        }
    }
}
