use telegram_bot::{UserId};
pub use crate::services::shopping_bot_message_service::ShoppingBotService;
pub use crate::services::telegram_message_send_service::TelegramMessageSendService;

pub mod shopping_bot_message_service;
mod einkaufen_handler;
pub mod telegram_message_send_service;



pub fn get_telegram_service(token: String, project_id: i64, client_ids: Vec<UserId>) -> ShoppingBotService {
    ShoppingBotService::new(token, project_id, client_ids)
}

pub fn get_message_send_service(token: &String) -> TelegramMessageSendService {
    TelegramMessageSendService::new(token)
}