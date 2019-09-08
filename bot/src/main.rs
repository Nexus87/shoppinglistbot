//#![feature(proc_macro_hygiene, decl_macro)]

extern crate byteorder;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate telegram_bot;
extern crate todoist;
extern crate tokio;
extern crate gotham;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate failure;

extern crate sled;
extern crate bincode;
extern crate core;

mod errors;
mod routes;
mod services;
mod storage;
mod middleware;

use errors::ShoppingListBotError;
use routes::get_routes;
use services::{get_telegram_service, get_message_send_service};
use simplelog::*;
use std::env;
use storage::get_storage;
use telegram_bot::{Integer, UserId};

fn env_var(key: &str) -> Result<String, ShoppingListBotError> {
    env::var(key).map_err(|x| ShoppingListBotError::InitError {
        missings_var: format!("{}: {}", key, x),
    })
}
fn read_env_vars() -> Result<(String, i64, Vec<UserId>, String), ShoppingListBotError> {
    let todoist_token = env_var("TODOIST_TOKEN")?;
    let project_id: i64 = env_var("PROJECT_ID")?
        .parse()
        .map_err(|x| ShoppingListBotError::new_parsing_error(String::from("PROJECT_ID"), format!("{}",x)))?;

    let client_ids: Result<Vec<UserId>, _> = env_var("CLIENT_IDS")
        .unwrap_or_else(|_| String::from(""))
        .split(',')
        .map(|x| x.parse::<Integer>())
        .map(|x: Result<Integer, _>| x.map(From::from))
        .collect();
    
    let client_ids = client_ids
        .map_err(|x| ShoppingListBotError::new_parsing_error(String::from("PROJECT_ID"), format!("{}",x)))?;
    
    let telegram_token = env_var("TELEGRAM_BOT_TOKEN")?;
    Ok((todoist_token, project_id, client_ids, telegram_token))
}

fn run() -> Result<(), ShoppingListBotError> {
    let db_path = "./my.db";
    let (todoist_token, project_id, client_ids, bot_token) = read_env_vars()?;

    let db = get_storage(&db_path);
    let telegram_message_service = get_telegram_service(todoist_token, project_id, client_ids, db);
    let message_service = get_message_send_service(&bot_token);
    gotham::start("0.0.0.0:7878", get_routes(  message_service, telegram_message_service));
    Ok(())
//    rocket::ignite()
//        .manage(telegram_message_service)
//        .manage(message_service)
//        .mount("/", get_routes())
//        .launch();
//    Ok(())
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
fn main() {
    init_logging();
    
    if let Err(e) = run() {
        error!("{}", e);
        panic!()
    }
}
