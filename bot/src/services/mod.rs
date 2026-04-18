use std::sync::Arc;
use crate::errors::ShoppingListBotError;
use crate::services::shopping_bot_message_service::ShoppingBotMessageService;
use crate::services::telegram_message_send_service::TelegramMessageSendService;
use crate::storage::Storage;
use teloxide::prelude::{Update, UserId};
use teloxide::types::Chat;

pub mod shopping_bot_message_service;
mod einkaufsliste_handler;
mod einkaufen_handler;
mod store_handler;
pub mod telegram_message_send_service;

pub trait TelegramMessageService {
    async fn handle_message(&self, update: &Update) -> Result<Option<(Chat, String)>, ShoppingListBotError>;
}

pub trait MessageSendService {
    async fn send_message(&self, chat: Chat, message: &String);
}
pub fn get_telegram_service(token: String, project_id: &String, client_ids: Vec<UserId>, db: Arc<dyn Storage>) -> ShoppingBotMessageService {
    ShoppingBotMessageService::new(token, project_id, client_ids, db)
}

pub fn get_message_send_service(token: &String) -> TelegramMessageSendService {
    TelegramMessageSendService::new(token)
}