use crate::errors::ShoppingListBotError;
use telegram_bot::{Api, MessageChat, prelude::*};

pub struct SendText {
    text: String,
    chat: MessageChat,
}

pub struct TelegramActor {
    api: Api
}

impl TelegramActor {
    pub fn new(token: &String) -> Self {
        let api = Api::configure(token).build().unwrap();
        TelegramActor {
            api
        }
    }

    async fn sendText(&mut self, msg: SendText) -> Result<(), ShoppingListBotError> {
        let chat = msg.chat;
        let message = msg.text;
        info!("Send message {} to {:?}", message, chat.id());
        let f = self.api.send(chat.text(&message))
            .await?;
        info!("Done sending message {} to {:?}", &message, chat.id());
        
        Ok(())
    }
}