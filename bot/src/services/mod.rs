use errors::ShoppingListBotError;
use telegram_bot::{Update, UserId, MessageChat};
use services::shopping_bot_message_service::ShoppingBotMessageService;
use services::telegram_message_send_service::TelegramMessageSendService;
use storage::Storage;
use std::panic::RefUnwindSafe;

mod shopping_bot_message_service;
mod einkaufen_handler;
mod store_handler;
mod telegram_message_send_service;



pub fn get_telegram_service(token: String, project_id: i64, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> ShoppingBotMessageService {
    ShoppingBotMessageService::new(token, project_id, client_ids, db)
}

pub fn get_message_send_service(token: &String) -> TelegramMessageSendService {
    TelegramMessageSendService::new(token)
}