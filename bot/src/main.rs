#![feature(proc_macro_hygiene, decl_macro)]

extern crate futures;
extern crate telegram_bot;
extern crate todoist;
extern crate tokio;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate rocket;

mod handler;

use rocket::State;
use std::result::Result::Ok;
use telegram_bot::types::Update;
use std::env;
use telegram_bot::*;
use handler::message_handler::MessageHandler;


#[post("/webhook", data = "<payload>")]
pub fn receive(payload: String, message_handler: State<MessageHandler>) -> Result<(), ()> {
    let payload = serde_json::from_str::<Update>(payload.as_str()).unwrap();


    if let UpdateKind::Message(message) = payload.kind {
        message_handler.handle(&message);
    }

    Ok(())
}

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let todoist_token = env::var("TODOIST_TOKEN").unwrap();
    let project_id: i64 = env::var("PROJECT_ID").unwrap().parse().unwrap();

    let client_ids: Vec<UserId> = env::var("CLIENT_IDS")
        .unwrap_or_else(|_| String::from(""))
        .split(',')
        .map(|x| x.parse::<Integer>().unwrap())
        .map(From::from)
        .collect();

    let api = Api::configure(token).build().unwrap();
    let message_handler = MessageHandler::new(todoist_token, project_id, client_ids);
//    let future = api.stream()
//        .filter(move |update| {
//            let id = match &update.kind {
//                UpdateKind::Message(m) => Some(m.from.id),
//                _ => None
//            };
//
//            if let Some(ref id) = id {
//                return client_ids.contains(&id)
//            }
//            false
//        })
//        .for_each(move |update| {
//        if let UpdateKind::Message(message) = update.kind {
//            message_handler.handle(&message);
//        }
//        Ok(())
//    })
//        .map_err(|_| ());

    rocket::ignite()
        .manage(api)
        .manage(message_handler)
        .mount("/", routes![receive])
        .launch();
}
