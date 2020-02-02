use crate::errors::ShoppingListBotError;
use telegram_bot::{Update, UserId, MessageChat};
use crate::services::shopping_bot_message_service::ShoppingBotMessageService;
use crate::services::telegram_message_send_service::TelegramMessageSendService;
use crate::storage::Storage;

mod shopping_bot_message_service;
mod einkaufen_handler;
mod store_handler;
mod telegram_message_send_service;

pub trait TelegramMessageService: Sync + Send{
    fn handle_message(&self, update: &Update) -> Result<Option<(MessageChat, String)>, ShoppingListBotError>;
}

pub trait MessageSendService: Sync + Send {
    fn send_message(&self, chat: MessageChat, message: &String);
}
pub fn get_telegram_service(token: String, project_id: i64, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> Box<dyn TelegramMessageService> {
    Box::new(ShoppingBotMessageService::new(token, project_id, client_ids, db))
}

pub fn get_message_send_service(token: &String) -> Box<dyn MessageSendService> {
    Box::new(TelegramMessageSendService::new(token))
}