use teloxide::Bot;
use teloxide::prelude::Requester;
use teloxide::types::Chat;

pub struct TelegramMessageSendService {
    api: Bot,
}

impl TelegramMessageSendService {
    pub fn new(token: &String) -> Self {
        let api = Bot::new(token);
        TelegramMessageSendService { api }
    }
}
impl super::MessageSendService for TelegramMessageSendService {
    async fn send_message(&self, chat: Chat, message: &String) {
        if let Err(e) = self.api.send_message(chat.id, message).await {
            error!("{}", e)
        };
    }
}
