use telegram_bot::{
    Api,
    MessageChat,
    prelude::*,
};
use std::panic::AssertUnwindSafe;
use std::sync::Arc;

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

    pub fn send_message(&self, chat: MessageChat, message: &String) {
        info!("Send message {} to {:?}", message, chat.id());
        let f = self.api.send(chat.text(message));
        let mut runtime = tokio::runtime::Runtime::new().expect("failed to start new Runtime");
        runtime
            .block_on(f)
            .expect("shutdown cannot error");
        info!("Done sending message {} to {:?}", message, chat.id());
    }
}
