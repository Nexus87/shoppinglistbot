use telegram_bot::{prelude::*, Api, MessageChat};

pub struct TelegramMessageSendService {
    api: Api,
}

impl TelegramMessageSendService {
    pub fn new(token: &String) -> Self {
        let api = Api::new(token);
        TelegramMessageSendService { api }
    }
}
impl super::MessageSendService for TelegramMessageSendService {
    async fn send_message(&self, chat: MessageChat, message: &String) {
        if let Err(e) = self.api.send(chat.text(message)).await {
            error!("{}", e)
        };
    }
}
