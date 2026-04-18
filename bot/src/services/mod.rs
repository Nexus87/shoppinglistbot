use crate::errors::ShoppingListBotError;
use telegram_bot::{Update, UserId, MessageChat};
use crate::services::shopping_bot_message_service::ShoppingBotMessageService;
use crate::services::telegram_message_send_service::TelegramMessageSendService;
use crate::storage::Storage;
use std::pin::Pin;
use std::future::Future;
pub mod shopping_bot_message_service;
mod einkaufsliste_handler;
mod einkaufen_handler;
mod store_handler;
pub mod telegram_message_send_service;

pub trait TelegramMessageService: Sync + Send {
    fn handle_message(&self, update: &Update) -> Pin<Box<dyn Future<Output = Result<Option<(MessageChat, String)>, ShoppingListBotError>> + Send>>;
}

pub trait MessageSendService: Sync + Send {
    async fn send_message(&self, chat: MessageChat, message: &String);
}
pub fn get_telegram_service(token: String, project_id: &String, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> ShoppingBotMessageService {
    ShoppingBotMessageService::new(token, project_id, client_ids, db)
}

pub fn get_message_send_service(token: &String) -> TelegramMessageSendService {
    TelegramMessageSendService::new(token)
}