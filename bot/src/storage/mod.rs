use telegram_bot::types::ChatId;
use storage::sled::SledStorage;

pub mod sled;
pub trait Storage: Send + Sync {
    fn get_last_update_id(&self, chat: ChatId) -> Option<i64>;
    fn set_last_update_id(&self, chat: ChatId, update_id: i64);
}

pub fn get_storage(path: &str) -> Box<dyn Storage> {
    Box::new(SledStorage::new(path))
}