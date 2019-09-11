use telegram_bot::{
    Api,
    MessageChat,
    prelude::*,
};
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use errors::ShoppingListBotError;
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

    pub fn send_message(&self, chat: MessageChat, message: &String) -> Box<dyn Future<Item=(), Error=ShoppingListBotError>+ Send>{
        info!("Send message {} to {:?}", message, chat.id());
        if message.is_empty() {
            return Box::new(future::ok(()))
        }
        
        let res = self.api.send(chat.text(message))
            .map(|_| ())
            .map_err(|e| e.into());
        Box::new(res)
    }
}
