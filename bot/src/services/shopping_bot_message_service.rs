use super::einkaufen_handler::EinkaufenCommandHandler;
use crate::errors::ShoppingListBotError;
use crate::services::store_handler::StoreCommandHandler;
use crate::services::TelegramMessageService;
use crate::storage::Storage;
use lazy_static::lazy_static;
use regex::Regex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use telegram_bot::types::Update;
use telegram_bot::types::UserId;
use telegram_bot::{Message, MessageChat, MessageKind, UpdateKind};
use todoist::shopping_list_api::TodoistApi;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Command {
    Config,
    Einkaufen,
    TestStore,
    TestGet,
    None,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "/config" => Command::Config,
            "/einkaufen" => Command::Einkaufen,
            "/store" => Command::TestStore,
            "/load" => Command::TestGet,
            _ => Command::None,
        }
    }
}
lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^(?P<command>/[^@]+)(@[^\s]*)?").unwrap();
    // static ref COMMAND_REGEX: Regex = Regex::new(r"^(\[^@]+)(@[^\s]*)?").unwrap();
}

fn parse_message(message: &MessageKind) -> Option<(Command, String)> {
    if let MessageKind::Text { ref data, .. } = message {
        let split: Vec<&str> = data.splitn(2, ' ').collect();
        let command = match COMMAND_REGEX.captures(split.get(0)?) {
            None => Command::None,
            Some(capture) => Command::from(&capture["command"]),
        };
        let args = match command {
            Command::None => data.clone(),
            _ => split.get(1).cloned().unwrap_or_default().to_string(),
        };

        return Some((command, args));
    }
    None
}

#[derive(Clone)]
pub struct ShoppingBotMessageService {
    client_ids: Vec<UserId>,
    einkaufen_handler: Arc<EinkaufenCommandHandler>,
    store_handler: Arc<StoreCommandHandler>,
    db: Arc<dyn Storage>,
}

impl ShoppingBotMessageService {
    pub fn new(
        token: String,
        project_id: i64,
        client_ids: Vec<UserId>,
        db: Box<dyn Storage>,
    ) -> Self {
        let api = TodoistApi::new(token);
        let db: Arc<dyn Storage> = Arc::from(db);
        ShoppingBotMessageService {
            client_ids,
            einkaufen_handler: Arc::new(EinkaufenCommandHandler::new(api, project_id)),
            db: db.clone(),
            store_handler: Arc::new(StoreCommandHandler::new(db)),
        }
    }

    pub async fn handle(&self, message: &Message) -> Option<String> {
        if !self.client_ids.contains(&message.from.id) {
            warn!("Unknown client: {:?}", message.from);
            return None;
        }
        if let Some((command, args)) = parse_message(&message.kind) {
            return match command {
                Command::Einkaufen => {
                    return self.einkaufen_handler.handle_message(&args).await.unwrap();
                }
                Command::TestStore => {
                    self.store_handler.handle_message(&args);
                    None
                }
                Command::TestGet => self.store_handler.handle_message(&args),
                _ => None,
            };
        }
        None
    }
}

impl TelegramMessageService for ShoppingBotMessageService {
    fn handle_message(
        &self,
        update: &Update,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Option<(MessageChat, String)>, ShoppingListBotError>> + Send,
        >,
    > {
        let update = update.clone();
        let api = self.clone();
        let result = async move {
            if let UpdateKind::Message(message) = update.kind {
                let last_update_id = api.db.get_last_update_id(message.chat.id())?;
                if let Some(id) = last_update_id {
                    info!("Last id: {}, current id: {}", id, update.id);
                    if id >= update.id {
                        return Ok(None);
                    }
                }
                api.db.set_last_update_id(message.chat.id(), update.id)?;
                return Ok(api
                    .handle(&message)
                    .await
                    .map(|m| (message.chat.clone(), m)));
            }
            Ok(None)
        };

        Box::pin(result)
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
    einkaufen_at:("/einkaufen@foo", (Command::Einkaufen, "")),
    config_args: ("/config bla bla", (Command::Config, "bla bla")),
    config_no_args: ("/config", (Command::Config, "")),
    none: ("bla bla", (Command::None, "bla bla")),
    einkaufen_no_slash: ("einkaufen bla bla", (Command::None, "einkaufen bla bla")),
}
