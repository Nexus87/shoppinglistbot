use telegram_bot::{
    Api,
    MessageChat,
    prelude::*,
};
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use crate::errors::ShoppingListBotError;
use futures::{Future, future};

#[derive(Clone)]
pub struct TelegramMessageSendService {
    api: Arc<AssertUnwindSafe<Api>>
}

impl TelegramMessageSendService {
    pub fn new(token: &String) -> Self {
        let api = AssertUnwindSafe(Api::configure(token).build().unwrap());
        TelegramMessageSendService {
            api: Arc::new(api)
        }
    }

    pub async fn send_message(&self, chat: MessageChat, message: &String) -> Result<(), ShoppingListBotError>{
        info!("Send message {} to {:?}", message, chat.id());
        if message.is_empty() {
            return Ok(());
        }
        
        self.api.send(chat.text(message)).await?;
        Ok(())
    }
}
