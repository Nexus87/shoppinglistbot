use telegram_bot::{
    Api,
    MessageChat,
    prelude::*
};

pub struct TelegramMessageSendService {
    api: Api
}

impl TelegramMessageSendService {
    pub fn new(token: &String) -> Self {
        let api = Api::configure(token).build().unwrap();
        TelegramMessageSendService {
            api
        }
    }
}
impl super::MessageSendService for TelegramMessageSendService {
    fn send_message(&self, chat: MessageChat, message: &String) {
        self.api.spawn(chat.text(message))
    }
}