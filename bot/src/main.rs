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

mod services;
mod storage;
mod routes;

use std::env;
use telegram_bot::*;
use routes::get_routes;
use storage::get_storage;
use services::get_telegram_service;


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
    let db = get_storage(&db_path);
    let telegram_message_service = get_telegram_service(todoist_token, project_id, client_ids, db);

    rocket::ignite()
        .manage(api)
        .manage(telegram_message_service)
        .mount("/", get_routes())
        .launch();
}
