use errors::ShoppingListBotError;
use telegram_bot::{Api, MessageChat, prelude::*};
use actix::prelude::*;

pub struct SendText {
    text: String,
    chat: MessageChat,
}

impl Message for SendText {
    type Result = Result<(), ShoppingListBotError>;
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
    type Result = Box<Future<Item = (), Error = ShoppingListBotError>>;
    fn handle(&mut self, msg: SendText, _: &mut Context<Self>) -> Box<Future<Item = (), Error = ShoppingListBotError>> {
        let chat = msg.chat;
        let message = msg.text;
        info!("Send message {} to {:?}", message, chat.id());
        let f = self.api.send(chat.text(&message))
            .map(move |_| {
                info!("Done sending message {} to {:?}", &message, chat.id());
                ()
            })
            .map_err(|e|e.into());
        Box::new(f)
    }
}