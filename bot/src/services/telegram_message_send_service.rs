use async_trait::async_trait;
use telegram_bot::{prelude::*, Api, MessageChat};

pub struct TelegramMessageSendService {
    api: Api,
}

impl TelegramMessageSendService {
    pub fn new(token: &String) -> Self {
        let api = Api::configure(token).build().unwrap();
        TelegramMessageSendService { api }
    }
}
#[async_trait]
impl super::MessageSendService for TelegramMessageSendService {
    async fn send_message(&self, chat: MessageChat, message: &String) {
        if let Err(e) = self.api.send(chat.text(message)).await {
            error!("{}", e)
        };
    }
}
