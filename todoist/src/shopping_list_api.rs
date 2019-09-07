use hyper_tls::HttpsConnector;
use hyper::{
    Body,
    client::HttpConnector,
    Client,
    Method,
    Request,
    header::HeaderValue,
};
use futures::future::Future;
use futures::stream::Stream;
use requests::GetProjectsRequest;
use types::{
    requests::{
        Task,
        Command,
        WriteResource
    },
    primitives::Integer,
    todoist::GetProjectsResponse,
};
use serde::Serialize;

const URL: &str = "https://todoist.com/api/v8/sync";

type TodoistClient = Client<HttpsConnector<HttpConnector>>;

pub struct TodoistApi {
    token: String,
    client: TodoistClient,
}

impl TodoistApi {
    pub fn new(token: String) -> TodoistApi {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        TodoistApi {
            token,
            client: Client::builder()
                .build(https),
        }
    }

    fn make_request<T> (&self, payload: &T) -> Box<Future<Item=hyper::Chunk, Error=hyper::Error> +Send> where T: Serialize  {
        let payload = serde_json::to_string(&payload).unwrap();
        let uri: hyper::Uri = URL.parse().unwrap();
        let mut req = Request::new(Body::from(payload));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let res = self.client.request(req)
            .and_then(|res| {
                println!("POST: {}", res.status());

                res.into_body().concat2()
            });
        Box::new(res)
    }
}

pub trait ShoppingListApi {
    fn get_projects(&self) -> Box<Future<Item=GetProjectsResponse, Error=hyper::Error>>;
    fn add_tasks(&self, texts: &[&str], project_id: Integer) -> Box<Future<Item=(), Error=hyper::Error> + Send>;
    fn add_task(&self, text: &str, project_id: Integer) -> Box<Future<Item=(), Error=hyper::Error> + Send> {
        let texts = [text];
        self.add_tasks(&texts, project_id)
    }
}

impl ShoppingListApi for TodoistApi {
    fn get_projects(&self) -> Box<Future<Item=GetProjectsResponse, Error=hyper::Error>> {
        let json = GetProjectsRequest {
            token: self.token.clone(),
            sync_token: "*".to_string(),
            resource_types: "[\"projects\"]".to_string(),
        };
        let result = self.make_request(&json)
            .map(move |body| {
                println!("{:#?}", body);
                let response = serde_json::from_slice::<GetProjectsResponse>(&body).unwrap();
                println!("{:#?}", response);
                response
            });
        Box::new(result)
    }

    fn add_tasks(&self, texts: &[&str], project_id: Integer) -> Box<Future<Item=(), Error=hyper::Error> + Send> {
        let commands: Vec<Command<Task>> = texts.iter()
            .map(|x| Task::new(x, project_id))
            .map(Command::new_add_task)
            .collect();

        let request = WriteResource::new(&commands, &self.token).unwrap();
        Box::new(self.make_request(&request).map(|_| ()))
    }
}