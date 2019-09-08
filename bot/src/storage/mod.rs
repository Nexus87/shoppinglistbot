use errors::ShoppingListBotError;
use telegram_bot::types::ChatId;
use storage::sled::SledStorage;
use std::panic::RefUnwindSafe;

pub mod sled;
pub trait Storage: Send + Sync + RefUnwindSafe{
    fn get_last_update_id(&self, chat: ChatId) -> Result<Option<i64>, ShoppingListBotError>;
    fn set_last_update_id(&self, chat: ChatId, update_id: i64) -> Result<(), ShoppingListBotError>;

    fn get_temp(&self, key: &str) -> Result<Option<String>, ShoppingListBotError>;
    fn set_temp(&self, key: &str, value: String) -> Result<(), ShoppingListBotError>;
}

pub fn get_storage(path: &str) -> Box<dyn Storage> {
    Box::new(SledStorage::new(path))
}