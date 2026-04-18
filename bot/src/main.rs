extern crate todoist;
extern crate tokio;
#[macro_use]
extern crate log;
extern crate simplelog;

extern crate core;
extern crate sled;

mod errors;
mod routes;
mod services;
mod storage;

use crate::services::shopping_bot_message_service::ShoppingBotMessageService;
use crate::services::telegram_message_send_service::TelegramMessageSendService;
use axum::Router;
use axum::routing::post;
use errors::ShoppingListBotError;
use routes::telegram_webhook;
use services::{get_message_send_service, get_telegram_service};
use simplelog::*;
use std::env;
use std::sync::Arc;
use teloxide::prelude::UserId;
use storage::get_storage;

fn env_var(key: &str) -> Result<String, ShoppingListBotError> {
    env::var(key).map_err(|x| ShoppingListBotError::InitError {
        missings_var: format!("{}: {}", key, x),
    })
}

fn read_env_vars() -> Result<(String, String, Vec<UserId>, String, u16), ShoppingListBotError> {
    let todoist_token = env_var("TODOIST_TOKEN")?;
    let project_id: String = env_var("PROJECT_ID")?;

    let client_ids: Result<Vec<UserId>, _> = env_var("CLIENT_IDS")
        .unwrap_or_else(|_| String::from(""))
        .split(',')
        .map(|x| x.trim().parse::<u64>().map(|x| {UserId(x)}))
        .collect();

    let client_ids = client_ids.map_err(|x| {
        ShoppingListBotError::new_parsing_error(String::from("CLIENT_IDS"), format!("{}", x))
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
struct AppState {
    telegram_service: ShoppingBotMessageService,
    message_send_service: TelegramMessageSendService,
}
async fn run() -> Result<(), ShoppingListBotError> {
    let db_path = "./my.db";
    let (todoist_token, project_id, client_ids, bot_token, port) = read_env_vars()?;

    let db = get_storage(&db_path);
    let telegram_service = get_telegram_service(todoist_token, &project_id, client_ids, db);
    let message_send_service = get_message_send_service(&bot_token);
    let shared_state = Arc::new(AppState {
        telegram_service,
        message_send_service,
    });
    let app = Router::new()
        .route("/webhook", post(telegram_webhook))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;
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
    if let Err(e) = TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ) {
        println!("{}", e);
    }
}

#[tokio::main]
async fn main() {
    init_logging();

    if let Err(e) = run().await {
        error!("{}", e);
        panic!()
    }
}
