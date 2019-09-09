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
use std::panic::AssertUnwindSafe;

const URL: &str = "https://todoist.com/api/v8/sync";

type TodoistClient = Client<HttpsConnector<HttpConnector>>;

pub struct TodoistApi {
    token: String,
    client: AssertUnwindSafe<TodoistClient>,
}

impl TodoistApi {
    pub fn new(token: String) -> TodoistApi {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder()
            .build(https);
        TodoistApi {
            token,
            client: AssertUnwindSafe(client),
        }
    }

    fn make_request (&self, payload: String) -> impl Future<Item=hyper::Chunk, Error=hyper::Error>  {
        let uri: hyper::Uri = URL.parse().unwrap();
        let mut req = Request::new(Body::from(payload));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        self.client.request(req)
            .and_then(|res| {
                println!("POST: {}", res.status());

                res.into_body().concat2()
            })
    }

    pub fn add_tasks(&self, texts: Vec<String>, project_id: Integer) -> impl Future<Item=Vec<String>, Error=hyper::Error> + 'static {
        let commands: Vec<Command<Task>> = texts.iter()
            .map(|x| Task::new(x, project_id))
            .map(Command::new_add_task)
            .collect();
        let token = self.token.clone();
        let request = WriteResource::new(&commands, &token).unwrap();
        let payload = serde_json::to_string(&request).unwrap();
        self.make_request(payload).map(move |_| texts)
    }
}

pub trait ShoppingListApi {
    fn get_projects(&self) -> Box<dyn Future<Item=GetProjectsResponse, Error=hyper::Error>>;
}

impl ShoppingListApi for TodoistApi {
    fn get_projects(&self) -> Box<dyn Future<Item=GetProjectsResponse, Error=hyper::Error>> {
        let json = GetProjectsRequest {
            token: self.token.clone(),
            sync_token: "*".to_string(),
            resource_types: "[\"projects\"]".to_string(),
        };
        let payload = serde_json::to_string(&json).unwrap();

        let result = self.make_request(payload)
            .map(move |body| {
                println!("{:#?}", body);
                let response = serde_json::from_slice::<GetProjectsResponse>(&body).unwrap();
                println!("{:#?}", response);
                response
            });
        Box::new(result)
    }
}