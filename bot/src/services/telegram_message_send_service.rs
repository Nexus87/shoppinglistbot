use errors::ShoppingListBotError;
use telegram_bot::{
    Api,
    MessageChat,
    prelude::*
};
use actix::prelude::*;
use tokio::prelude::*;

pub struct SendText {
    text: String,
    chat: MessageChat
}

impl Message for SendText {
    type Result = Future<Result<(), ShoppingListBotError>>;
}

pub struct TelegramActor {
    api: Api
}

impl Actor for TelegramActor {
    type Context = Context<Self>;
}

impl TelegramActor {
    pub fn new(token: &String) -> Self {
        let api = Api::configure(token).build().unwrap();
        TelegramActor {
            api
        }
    }
}

impl Handler<SendText> for TelegramActor {
    type Result = Future<Result<(), ShoppingListBotError>>;
    fn handle(&mut self, msg: SendText, _: &mut Context<Self>) -> Self::Result {
        let chat = msg.chat;
        let message = msg.text;
        info!("Send message {} to {:?}", message, chat.id());
        let f = self.api.send(chat.text(message));
        info!("Done sending message {} to {:?}", message, chat.id());
        f
    }
}