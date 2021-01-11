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

extern crate bincode;
extern crate core;
extern crate sled;

mod errors;
mod routes;
mod services;
mod storage;

use actix_web::{web, App, HttpServer};
use errors::ShoppingListBotError;
use routes::telegram_webhook;
use services::{get_message_send_service, get_telegram_service};
use simplelog::*;
use std::env;
use storage::get_storage;
use telegram_bot::{Integer, UserId};

fn env_var(key: &str) -> Result<String, ShoppingListBotError> {
    env::var(key).map_err(|x| ShoppingListBotError::InitError {
        missings_var: format!("{}: {}", key, x),
    })
}

fn read_env_vars() -> Result<(String, i64, Vec<UserId>, String, u16), ShoppingListBotError> {
    let todoist_token = env_var("TODOIST_TOKEN")?;
    let project_id: i64 = env_var("PROJECT_ID")?.parse().map_err(|x| {
        ShoppingListBotError::new_parsing_error(String::from("PROJECT_ID"), format!("{}", x))
    })?;

    let client_ids: Result<Vec<UserId>, _> = env_var("CLIENT_IDS")
        .unwrap_or_else(|_| String::from(""))
        .split(',')
        .map(|x| x.parse::<Integer>())
        .map(|x: Result<Integer, _>| x.map(From::from))
        .collect();

    let client_ids = client_ids.map_err(|x| {
        ShoppingListBotError::new_parsing_error(String::from("PROJECT_ID"), format!("{}", x))
    })?;

    let telegram_token = env_var("TELEGRAM_BOT_TOKEN")?;
    let port = env_var("PORT")
        .and_then(|x| {
            x.parse::<u16>().map_err(|_| {
                ShoppingListBotError::new_parsing_error(String::from(""), String::from(""))
            })
        })
        .unwrap_or_else(|_| 3030);
    println!("{}", port);
    Ok((todoist_token, project_id, client_ids, telegram_token, port))
}

async fn run() -> Result<(), ShoppingListBotError> {
    let db_path = "./my.db";
    let (todoist_token, project_id, client_ids, bot_token, port) = read_env_vars()?;

    let db = get_storage(&db_path);
    let telegram_message_service = get_telegram_service(todoist_token, project_id, client_ids, db);
    let message_service = get_message_send_service(&bot_token);

    HttpServer::new(move || {
        App::new()
            .data(telegram_message_service.clone())
            .data(message_service.clone())
            .route("/webhook", web::to(telegram_webhook))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;
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
        _ => LevelFilter::Info,
    };
    if let Err(e) = TermLogger::init(log_level, Config::default()) {
        println!("{}", e);
    }
}

#[actix_web::main]
async fn main() {
    init_logging();

    if let Err(e) = run().await {
        error!("{}", e);
        panic!()
    }
}
