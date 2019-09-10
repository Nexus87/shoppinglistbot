use std::sync::Arc;

use futures::Future;
use telegram_bot::{Message, MessageChat, MessageKind, UpdateKind};
use telegram_bot::types::Update;
use telegram_bot::types::UserId;

use errors::ShoppingListBotError;
use storage::Storage;
use todoist::shopping_list_api::TodoistApi;
//        return Box::new(res);

use super::einkaufen_handler::EinkaufenCommandHandler;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Command {
    Config,
    Einkaufen,
    None,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "/config" => Command::Config,
            "/einkaufen" => Command::Einkaufen,
            _ => Command::None,
        }
    }
}

fn parse_message(message: &MessageKind) -> Option<(Command, String)> {
    if let MessageKind::Text { ref data, .. } = message {
        let split: Vec<&str> = data.splitn(2, ' ').collect();
        let command = Command::from(*split.get(0)?);
        let args = match command {
            Command::None => data.clone(),
            _ => split.get(1).cloned().unwrap_or_default().to_string(),
        };

        return Some((command, args));
    }
    None
}

#[derive(Clone)]
pub struct ShoppingBotService {
    client_ids: Vec<UserId>,
    einkaufen_handler: EinkaufenCommandHandler,
    db: Arc<dyn Storage>,

}


impl ShoppingBotService {
    pub fn new(token: String, project_id: i64, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> Self {
        let api = TodoistApi::new(token);
        let db: Arc<dyn Storage> = Arc::from(db);
        ShoppingBotService {
            client_ids,
            einkaufen_handler: EinkaufenCommandHandler::new(api, project_id),
            db: db.clone(),
        }
    }

    pub fn handle(&self, message: &Message) -> Box<dyn Future<Item=String, Error=ShoppingListBotError> + Send> {
        let default = Box::new(futures::empty());
        let einkaufen_handler = self.einkaufen_handler.clone();
        if !self.client_ids.contains(&message.from.id) {
            warn!("Unknown client: {:?}", message.from);
            return default;
        }
        if let Some((command, args)) = parse_message(&message.kind) {
            info!("Command {:?}", command);
            match command {
                Command::Einkaufen => return Box::new(einkaufen_handler.handle_message(args)),
                _ => {
                    info!("Unknown command {:?}", command);
                    return default;
                }
            }
        }
        default
    }
    pub fn handle_message(self, update: Update) -> Box<dyn Future<Item=(MessageChat, String), Error=ShoppingListBotError> + Send + 'static> {
        let update_id = update.id;
        if let UpdateKind::Message(message) = update.kind {
            let db = self.db.clone();
            let res = futures::done(db.get_last_update_id(message.chat.id()))
                .and_then(move |last_update_id| {
                    if let Some(id) = last_update_id {
                        info!("Last id: {}, current id: {}", id, update_id);
                        if id >= update_id {
                            let r: Box<dyn Future<Item=Message, Error=ShoppingListBotError> + Send+ 'static> = Box::new(futures::empty());
                            return r;
                        }
                    }
                    let res = futures::done(db.set_last_update_id(message.chat.id(), update_id))
                        .map(|_| message);
                    Box::new(res)
                })
                .and_then(move |message| {
                    self.handle(&message)
                        .map(move |m| (message.chat.clone(), m))
                })
                .map_err(|err| err.into());
            return Box::new(res);
        }
        Box::new(futures::empty())
    }
}

macro_rules! parse_message_test {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            fn to_message(s: &'static str) -> MessageKind{
                MessageKind::Text {
                    data: s.to_string(),
                    entities: vec![]
                }
            }
            let (input, expected) = $value;
            let (expected_command, expected_args) = expected;
            let expected_args = expected_args.to_string();
            let input = to_message(input);
            let (command, args) = parse_message(&input).unwrap();

            assert_eq!(expected_command, command);
            assert_eq!(expected_args, args);
        }
    )*
    }
}

parse_message_test! {
    einkaufen_args: ("/einkaufen bla bla", (Command::Einkaufen, "bla bla")),
    einkaufen_no_args: ("/einkaufen", (Command::Einkaufen, "")),
    config_args: ("/config bla bla", (Command::Config, "bla bla")),
    config_no_args: ("/config", (Command::Config, "")),
    load_no_args: ("/load", (Command::TestGet, "")),
    store_args: ("/store bla bla", (Command::TestStore, "bla bla")),
    none: ("bla bla", (Command::None, "bla bla")),
}
