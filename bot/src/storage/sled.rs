use errors::ShoppingListBotError;
use sled::Db;
use storage::Storage;
use telegram_bot::types::ChatId;
use bincode::{serialize, deserialize};
use serde::de::DeserializeOwned;
use std::panic::AssertUnwindSafe;

pub struct SledStorage {
    db: AssertUnwindSafe<Db>,
}

impl SledStorage {
    pub fn new(path: &str) -> Self {
        SledStorage {
            db: AssertUnwindSafe(Db::start_default(path).unwrap()),
        }
    }
    fn chat_id_to_u8(x: ChatId) -> [u8; 8] {
        let i: i64 = x.into();

        let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(i) };
        raw_bytes
    }
    
    fn deserialize<'a, T>(value: &Vec<u8>) -> Result<T, ShoppingListBotError> where T: DeserializeOwned {
        let ret: T = deserialize(value)?;
        Ok(ret)
    }
}

impl Storage for SledStorage {
    fn get_last_update_id(&self, chat: ChatId) -> Result<Option<i64>, ShoppingListBotError> {
        let chat = SledStorage::chat_id_to_u8(chat);
        let update = self.db.get(&chat)?;

        if let Some(update) = update {
            // let update: i64 = String::from_utf8(update.to_vec()).unwrap().parse().unwrap();
            let update_id = String::from_utf8(update.to_vec())
                .map(|x| x.parse::<i64>())
                .map_err(|_| ShoppingListBotError::StorageError {
                    error_message: String::from("cannot parse value"),
                })?
                .map_err(|_| ShoppingListBotError::StorageError {
                    error_message: String::from("cannot parse value"),
                })?;
            trace!("Read update_id {}", update_id);
            return Ok(Some(update_id))
        }
        trace!("Key not found");
        Ok(None)
    }

    fn set_last_update_id(&self, chat: ChatId, update_id: i64) -> Result<(), ShoppingListBotError> {
        let chat = SledStorage::chat_id_to_u8(chat);
        let update_id: String = update_id.to_string();
        trace!("Write update_id {}", update_id);

        self.db.set(&chat, update_id.as_bytes().to_vec())?;
        Ok(())
    }

    fn get_temp(&self, key: &str) -> Result<Option<String>, ShoppingListBotError> {
        let key = serialize(key)?;
        let value = self.db.get(key)?;
        let ret = match value {
            None => None,
            Some(v) => Some(SledStorage::deserialize(&v.to_vec())?),
        };
        Ok(ret)
    }

    fn set_temp(&self, key: &str, value: String) -> Result<(), ShoppingListBotError> {
        let key = serialize(key)?;
        let value = serialize(&value)?;
        self.db.set(key, value)?;
        Ok(())
    }
}
