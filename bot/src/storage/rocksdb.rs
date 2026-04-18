use std::str::FromStr;
use rocksdb::DB;
use telegram_bot::ChatId;
use crate::errors::ShoppingListBotError;
use crate::storage::Storage;

pub struct RocksDbStorage {
    pub db: DB
}

impl RocksDbStorage {
    pub fn new(path: &str) -> Self {
        let db = DB::open_default(path).unwrap();
        Self { db }
    }
}
impl Storage for RocksDbStorage {
    fn get_last_update_id(&self, chat: ChatId) -> Result<Option<i64>, ShoppingListBotError> {
        let value = self.db.get(format!("last_update_id:{}", chat))?;
        if value == None {
            return Ok(None);
        }
        let update_id = String::from_utf8(value.unwrap())
            .map(|s| i64::from_str(&s))
            .map_err(|_| ShoppingListBotError::StorageError {
                error_message: String::from("cannot parse value"),
            })?
            .map_err(|_| ShoppingListBotError::StorageError {
                error_message: String::from("cannot parse value"),
            })?;
        trace!("Read update_id {}", update_id);
        Ok(Some(update_id))

    }

    fn set_last_update_id(&self, chat: ChatId, update_id: i64) -> Result<(), ShoppingListBotError> {
        self.db.put(format!("last_update_id:{}", chat), update_id.to_string())?;
        Ok(())
    }

    fn get_temp(&self, key: &str) -> Result<Option<String>, ShoppingListBotError> {
        match self.db.get(key)? {
            None => Ok(None),
            Some(x) => Ok(Some(String::from_utf8(x)?))
        }
    }

    fn set_temp(&self, key: &str, value: String) -> Result<(), ShoppingListBotError> {
        self.db.put(key, &value)?;
        Ok(())
    }
}