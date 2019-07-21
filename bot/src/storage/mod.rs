use actix::prelude::*;
use errors::ShoppingListBotError;
use sled::Db;
use telegram_bot::types::ChatId;
use bincode::{serialize, deserialize};
use serde::de::DeserializeOwned;

pub struct SledActor {
    db: Db,
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
}

impl Actor for SledActor {
    type Context = SyncContext<Self>;
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


impl Message for CheckAndUpdate {
    type Result = Result<bool, ShoppingListBotError>;
}
impl Message for Write {
    type Result = Result<(), ShoppingListBotError>;
}
impl Message for Read {
    type Result = Result<Option<String>, ShoppingListBotError>;
}

impl Handler<CheckAndUpdate> for SledActor {
    type Result = Result<bool, ShoppingListBotError>;
    fn handle(&mut self, msg: CheckAndUpdate, _: &mut SyncContext<Self>) -> Result<bool, ShoppingListBotError> {
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
}
impl Handler<Write> for SledActor {
    type Result = Result<(), ShoppingListBotError>;

    fn handle(&mut self, msg: Write, _: &mut SyncContext<Self>) -> Result<(), ShoppingListBotError> {
        let key = serialize(&msg.chat_id)?;
        let value = serialize(&msg.value)?;
        self.db.set(key, value)?;
        Ok(())
    }
}
impl Handler<Read> for SledActor {
    type Result = Result<Option<String>, ShoppingListBotError>;

    fn handle(&mut self, msg: Read, _: &mut SyncContext<Self>) -> Result<Option<String>, ShoppingListBotError> {
        let key = serialize(&msg.chat_id)?;
        let value = self.db.get(key)?;
        let ret = match value {
            None => None,
            Some(v) => Some(SledActor::deserialize(&v.to_vec())?),
        };
        Ok(ret)
    }
}



