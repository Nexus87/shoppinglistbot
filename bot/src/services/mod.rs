use telegram_bot::{UserId};
pub use services::shopping_bot_message_service::ShoppingBotService;
pub use services::telegram_message_send_service::TelegramMessageSendService;
use storage::Storage;

pub mod shopping_bot_message_service;
mod einkaufen_handler;
pub mod telegram_message_send_service;



pub fn get_telegram_service(token: String, project_id: i64, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> ShoppingBotService {
    ShoppingBotService::new(token, project_id, client_ids, db)
}

pub fn get_message_send_service(token: &String) -> TelegramMessageSendService {
    TelegramMessageSendService::new(token)
}