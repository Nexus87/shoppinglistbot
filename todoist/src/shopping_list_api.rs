use hyper_tls::HttpsConnector;
use hyper::{
    Body,
    client::HttpConnector,
    Client,
    Method,
    Request,
    header::HeaderValue,
};
use bytes::buf::BufExt as _;
use crate::types::{
    primitives::Integer,
    requests::{
        Command,
        Task,
        WriteResource,
        GetProjectsRequest
    },
    todoist::{
        GetProjectsResponse
    }
};
use std::panic::AssertUnwindSafe;

const URL: &str = "https://todoist.com/api/v8/sync";

type TodoistClient = Client<HttpsConnector<HttpConnector>>;

pub struct ShoppingListApi {
    token: String,
    client: AssertUnwindSafe<TodoistClient>,
}

impl ShoppingListApi {
    pub fn new(token: String) -> ShoppingListApi {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build(https);
            ShoppingListApi {
            token,
            client: AssertUnwindSafe(client),
        }
    }

    async fn make_request (&self, payload: String) -> Result<impl hyper::body::Buf, hyper::Error>  {
        let uri: hyper::Uri = URL.parse().unwrap();
        let mut req = Request::new(Body::from(payload));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let res = self.client.request(req).await?;
        //println!("POST: {}", res.status());

        Ok(hyper::body::aggregate(res).await?)
    }

    pub async fn add_tasks(&self, texts: Vec<String>, project_id: Integer) -> Result<Vec<String>, hyper::Error> {
        let commands: Vec<Command<Task>> = texts.iter()
            .map(|x| Task::new(x, project_id))
            .map(Command::new_add_task)
            .collect();
        let token = self.token.clone();
        let request = WriteResource::new(&commands, &token).unwrap();
        let payload = serde_json::to_string(&request).unwrap();
        self.make_request(payload).await?;
        Ok(texts)
    }

    pub async fn get_projects(&self) -> Result<GetProjectsResponse, hyper::Error> {
        let json = GetProjectsRequest {
            token: self.token.clone(),
            sync_token: "*".to_string(),
            resource_types: "[\"projects\"]".to_string(),
        };
        let payload = serde_json::to_string(&json).unwrap();

        let body = self.make_request(payload).await?;
        let response = serde_json::from_reader(body.reader()).unwrap();
        println!("{:#?}", response);
        Ok(response)
    }
}