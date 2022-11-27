use crate::errors::ShoppingListBotError;
use sled::Db;
use telegram_bot::types::ChatId;
use bincode::{serialize, deserialize};
use serde::de::DeserializeOwned;

pub struct SledActor {
    db: Db,
}

pub struct CheckAndUpdate {
    pub chat_id: ChatId,
    pub update_id: i64
}

pub struct Write {
    pub chat_id: ChatId,
    pub value: String
}
pub struct Read {
    pub chat_id: ChatId
}

impl SledActor {
    pub fn new(path: &str) -> Self {
        SledActor {
            db: Db::start_default(path).unwrap(),
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
    fn serialize_i64(value: i64) -> Result<Vec<u8>, ShoppingListBotError> {
        let ret = serialize(&value)?;
        Ok(ret)
    }
    
    fn checkAndUpdate(&self, msg: CheckAndUpdate) ->  Result<bool, ShoppingListBotError> {
        let chat = SledActor::chat_id_to_u8(msg.chat_id);
        let update_id = SledActor::serialize_i64(msg.update_id)?;
        let previous = self.db.get(chat)?;
        if let Some(previous_id) = &previous  {
            let previous_id: i64 = SledActor::deserialize(&previous_id.to_vec())?;
            if previous_id >= msg.update_id{
                return Ok(false)
            }
        }

        self.db.cas(chat, previous, Some(update_id))?.unwrap();
        Ok(true)
    }

    fn setValue(&self, msg: Write) -> Result<(), ShoppingListBotError> {
        let key = serialize(&msg.chat_id)?;
        let value = serialize(&msg.value)?;
        self.db.insert(key, value)?;
        Ok(())
    }
    
    fn readValue(&self, msg: Read) -> Result<Option<String>, ShoppingListBotError> {
        let key = serialize(&msg.chat_id)?;
        let value = self.db.get(key)?;
        let ret = match value {
            None => None,
            Some(v) => Some(SledActor::deserialize(&v.to_vec())?),
        };
        Ok(ret)
    }
}



