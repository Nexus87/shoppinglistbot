use errors::ShoppingListBotError;
use telegram_bot::{Update, UserId};
use services::shopping_bot_message_service::ShoppingBotMessageService;
use storage::Storage;

mod shopping_bot_message_service;
mod einkaufen_handler;


pub trait TelegramMessageService: Sync + Send{
    fn handle_message(&self, update: &Update) -> Result<(), ShoppingListBotError>;
}

pub fn get_telegram_service(token: String, project_id: i64, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> Box<dyn TelegramMessageService> {
    Box::new(ShoppingBotMessageService::new(token, project_id, client_ids, db))
}