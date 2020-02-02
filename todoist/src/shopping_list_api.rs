use hyper_tls::HttpsConnector;
use hyper::{
    Body,
    client::HttpConnector,
    Client,
    Method,
    Request,
    header::HeaderValue,
};
use bytes::buf::ext::BufExt;
use futures::future::Future;
use crate::requests::GetProjectsRequest;
use crate::types::{
    requests::{
        Task,
        Command,
        WriteResource
    },
    primitives::Integer,
    todoist::GetProjectsResponse,
};

const URL: &str = "https://todoist.com/api/v8/sync";

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
            client: Client::builder()
                .build(https),
        }
    }

    async fn make_request (&self, payload: String) -> Result<impl hyper::body::Buf, hyper::Error> {
        let uri: hyper::Uri = URL.parse().unwrap();
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

    async fn get_projects(&self) -> Result<GetProjectsResponse, hyper::Error> {
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

    async fn add_tasks(&self, texts: &[&str], project_id: Integer) -> Result<(), hyper::Error> {
        let commands: Vec<Command<Task>> = texts.iter()
            .map(|x| Task::new(x, project_id))
            .map(Command::new_add_task)
            .collect();

        let request = WriteResource::new(&commands, &self.token).unwrap();
        let payload = serde_json::to_string(&request).unwrap();
        self.make_request(payload).await?;
        Ok(())
    }
}