use telegram_bot::types::ChatId;

pub mod rocksdb;

pub trait Storage {
    fn get_last_update_id(&self, chat: ChatId) -> Option<i64>;
    fn set_last_update_id(&self, chat: ChatId, update_id: i64);
}