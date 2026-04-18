use std::sync::Arc;
use teloxide::prelude::ChatId;
use teloxide::types::UpdateId;
use crate::errors::ShoppingListBotError;
use crate::storage::rocksdb::RocksDbStorage;

pub mod rocksdb;

pub trait Storage: Send + Sync {
    fn get_last_update_id(&self, chat: ChatId) -> Result<Option<UpdateId>, ShoppingListBotError>;
    fn set_last_update_id(&self, chat: ChatId, update_id: UpdateId) -> Result<(), ShoppingListBotError>;

    fn get_temp(&self, key: &str) -> Result<Option<String>, ShoppingListBotError>;
    fn set_temp(&self, key: &str, value: String) -> Result<(), ShoppingListBotError>;
}

pub fn get_storage(path: &str) -> Arc<dyn Storage> {
    Arc::new(RocksDbStorage::new(path))
}