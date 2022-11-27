extern crate byteorder;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate telegram_bot;
extern crate todoist;
extern crate tokio;
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate failure;

extern crate sled;
extern crate bincode;
extern crate core;
extern crate actix_web;
extern crate actix;

mod errors;
mod routes;
mod services;
mod storage;

use errors::ShoppingListBotError;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use simplelog::*;
use std::env;
use telegram_bot::{Integer, UserId};
use actix::{SyncArbiter, Addr, Actor};
use services::telegram_message_send_service::TelegramActor;
use storage::SledActor;
use services::shopping_bot_message_service::ShoppingBotMessageService;

fn env_var(key: &str) -> Result<String, ShoppingListBotError> {
    env::var(key).map_err(|x| ShoppingListBotError::InitError {
        missing_var: format!("{}: {}", key, x),
    })
}


fn read_env_vars() -> Result<(String, i64, Vec<UserId>, String), ShoppingListBotError> {
    let todoist_token = env_var("TODOIST_TOKEN")?;
    let project_id: i64 = env_var("PROJECT_ID")?
        .parse()
        .map_err(|x| ShoppingListBotError::new_parsing_error(String::from("PROJECT_ID"), format!("{}", x)))?;

    let client_ids: Result<Vec<UserId>, _> = env_var("CLIENT_IDS")
        .unwrap_or_else(|_| String::from(""))
        .split(',')
        .map(|x| x.parse::<Integer>())
        .map(|x: Result<Integer, _>| x.map(From::from))
        .collect();

    let client_ids = client_ids
        .map_err(|x| ShoppingListBotError::new_parsing_error(String::from("PROJECT_ID"), format!("{}", x)))?;

    let telegram_token = env_var("TELEGRAM_BOT_TOKEN")?;
    Ok((todoist_token, project_id, client_ids, telegram_token))
}

async fn run() -> Result<(), ShoppingListBotError> {
    let db_path = "./my.db";
    let (todoist_token, project_id, client_ids, bot_token) = read_env_vars()?;

    let app = move || {
        let todoist_token = todoist_token.clone();
        let client_ids = client_ids.clone();
        let db = SyncArbiter::start(1, move || SledActor::new(&db_path));
        let telegram_message_service: Addr<TelegramActor> = TelegramActor::new(&bot_token).start();
        let shopping_bot_service = ShoppingBotMessageService::new(todoist_token, project_id, client_ids, db).start();
        App::new()
            .data(telegram_message_service)
            .data(shopping_bot_service)
            .wrap(Logger::default())
            .service(web::resource("/webhook").route(web::post()).to_async(routes::telegram_webhook))
    };
    HttpServer::new(app).bind("localhost:8088")?.run();
    Ok(())
}

fn init_logging() {
    let level_string = &env::var("LOG_LEVEL").unwrap_or_default()[..];
    let log_level = match level_string {
        "TRACE" => LevelFilter::Trace,
        "DEBUG" => LevelFilter::Debug,
        "INFO" => LevelFilter::Info,
        "WARN" => LevelFilter::Warn,
        "ERROR" => LevelFilter::Error,
        "OFF" => LevelFilter::Off,
        _ => LevelFilter::Info
    };
    if let Err(e) = SimpleLogger::init(log_level, Config::default()) {
        println!("{:?}", e);
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    if let Err(e) = run() {
        error!("{}", e);
        panic!()
    }
}
