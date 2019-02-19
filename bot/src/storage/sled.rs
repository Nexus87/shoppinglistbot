use sled::Db;
use storage::Storage;
use telegram_bot::types::ChatId;

pub struct SledStorage {
    db: Db
}

impl SledStorage {
    pub fn new(path: &str) -> Self {
        SledStorage {
            db: Db::start_default(path).unwrap()
        }
    }
    fn chat_id_to_u8(x: ChatId) -> [u8; 8] {
        let i: i64 = x.into();

        let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(i) };
        raw_bytes
    }
}

impl Storage for SledStorage {
    fn get_last_update_id(&self, chat: ChatId) -> Option<i64> {
        let chat = SledStorage::chat_id_to_u8(chat);
        let update = self.db.get(&chat)
            .unwrap();
        let update: i64 = String::from_utf8(update?.to_vec()).unwrap().parse().unwrap();
        println!("Read update_id {}", update);
        Some(update)
    }

    fn set_last_update_id(&self, chat: ChatId, update_id: i64) {
        let chat = SledStorage::chat_id_to_u8(chat);
        let update_id: String = update_id.to_string();
        println!("Write update_id {}", update_id);

        self.db.set(&chat, update_id.as_bytes().to_vec())
            .unwrap();
    }
}