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
use types::todoist::GetProjectsResponse;

const URL: &'static str = "https://todoist.com/api/v7/sync";

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
}

pub trait ShoppingListApi {
    fn get_projects(&self) -> Box<Future<Item=GetProjectsResponse, Error=hyper::Error>>;
}

impl ShoppingListApi for TodoistApi {
    fn get_projects(&self) -> Box<Future<Item=GetProjectsResponse, Error=hyper::Error>> {
        let json = GetProjectsRequest {
            token: self.token.clone(),
            sync_token: "*".to_string(),
            resource_types: "[\"projects\"]".to_string(),
        };
        let payload = serde_json::to_string(&json).unwrap();
        let uri: hyper::Uri = URL.parse().unwrap();
        let mut req = Request::new(Body::from(payload));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let result = self.client.request(req)
            .and_then(|res| {
                println!("POST: {}", res.status());

                res.into_body().concat2()
            })
            .map(move |body| {
                println!("{:#?}", body);
                let response = serde_json::from_slice::<GetProjectsResponse>(&body).unwrap();
                println!("{:#?}", response);
                response
            });
        Box::new(result)
    }
}