use crate::errors::ShoppingListBotError;
use telegram_bot::{Update, UserId, MessageChat};
use crate::services::shopping_bot_message_service::ShoppingBotMessageService;
use crate::services::telegram_message_send_service::TelegramMessageSendService;
use crate::storage::Storage;
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;
mod shopping_bot_message_service;
mod einkaufen_handler;
mod store_handler;
mod telegram_message_send_service;

pub trait TelegramMessageService: Sync + Send {
    fn handle_message(&self, update: &Update) -> Pin<Box<dyn Future<Output = Result<Option<(MessageChat, String)>, ShoppingListBotError>> + Send>>;
}

pub trait MessageSendService: Sync + Send {
    fn send_message(&self, chat: MessageChat, message: &String);
}
pub fn get_telegram_service(token: String, project_id: i64, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> Arc<dyn TelegramMessageService + Send> {
    Arc::new(ShoppingBotMessageService::new(token, project_id, client_ids, db))
}

pub fn get_message_send_service(token: &String) -> Arc<dyn MessageSendService + Send> {
    Arc::new(TelegramMessageSendService::new(token))
}