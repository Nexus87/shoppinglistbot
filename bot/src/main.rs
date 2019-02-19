#![feature(proc_macro_hygiene, decl_macro)]

extern crate byteorder;
extern crate futures;
extern crate telegram_bot;
extern crate todoist;
extern crate tokio;
extern crate serde_json;
extern crate serde;
extern crate rocket_contrib;

#[macro_use]
extern crate rocket;
extern crate sled;

mod handler;
mod storage;

use rocket::State;
use std::result::Result::Ok;
use telegram_bot::types::Update;
use std::env;
use telegram_bot::*;
use handler::message_handler::MessageHandler;
use rocket_contrib::json::Json;
use storage::sled::SledStorage;
use storage::Storage;

#[post("/webhook", format = "json", data = "<payload>")]
pub fn receive(payload: Json<Update>, message_handler: State<MessageHandler>, db: State<SledStorage>) -> Result<(), ()> {
    
    if let UpdateKind::Message(message) = &payload.kind {
        let last_update_id = db.get_last_update_id(message.chat.id());
        if let Some(id) = last_update_id {
            if id >= payload.id {
                return Ok(())
            }
        }
        db.set_last_update_id(message.chat.id(), payload.id);
        message_handler.handle(message);
        
    }

    Ok(())
}

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let todoist_token = env::var("TODOIST_TOKEN").unwrap();
    let project_id: i64 = env::var("PROJECT_ID").unwrap().parse().unwrap();
    let db_path = "./my.db";
    
    let client_ids: Vec<UserId> = env::var("CLIENT_IDS")
        .unwrap_or_else(|_| String::from(""))
        .split(',')
        .map(|x| x.parse::<Integer>().unwrap())
        .map(From::from)
        .collect();

    let api = Api::configure(token).build().unwrap();
    let message_handler = MessageHandler::new(todoist_token, project_id, client_ids);
    let db = SledStorage::new(&db_path);
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
        .manage(db)
        .manage(message_handler)
        .mount("/", routes![receive])
        .launch();
}
