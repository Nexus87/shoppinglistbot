use errors::ShoppingListBotError;
use super::einkaufen_handler::EinkaufenCommandHandler;
use todoist::shopping_list_api::TodoistApi;
use telegram_bot::types::UserId;
use services::TelegramMessageService;
use telegram_bot::types::Update;
use storage::Storage;
use telegram_bot::{UpdateKind, Message, MessageChat, MessageKind};
use services::store_handler::StoreCommandHandler;
use std::sync::Arc;

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

pub struct ShoppingBotMessageService {
    client_ids: Vec<UserId>,
    einkaufen_handler: EinkaufenCommandHandler,
    store_handler: StoreCommandHandler,
    db: Arc<dyn Storage>
}

impl ShoppingBotMessageService {
    pub fn new(token: String, project_id: i64, client_ids: Vec<UserId>, db: Box<dyn Storage>) -> Self {
        let api = TodoistApi::new(token);
        let db: Arc<dyn Storage> = Arc::from(db);
        ShoppingBotMessageService {
            client_ids,
            einkaufen_handler: EinkaufenCommandHandler::new(api, project_id),
            db: db.clone(),
            store_handler: StoreCommandHandler::new(db),
        }
    }

    pub fn handle(&self, message: &Message) -> Option<String> {
        if !self.client_ids.contains(&message.from.id) {
            warn!("Unknown client: {:?}", message.from);
            return None;
        }
        if let Some((command, args)) = parse_message(&message.kind) {
            info!("Command {:?}", command);
            match command { 
                Command::Einkaufen => return self.einkaufen_handler.handle_message(&args),
                Command::TestStore => {
                    self.store_handler.handle_message_store(&args);
                    return None;
                },
                Command::TestGet => return self.store_handler.handle_message_load(),
                _ => {
                    info!("Unknown command {:?}", command);
                    return None
                }
            }
        }
        None
    }
}

impl TelegramMessageService for ShoppingBotMessageService {
    fn handle_message(&self, update: &Update) -> Result<Option<(MessageChat, String)>, ShoppingListBotError> {
        if let UpdateKind::Message(message) = &update.kind {
            let last_update_id = self.db.get_last_update_id(message.chat.id())?;
            if let Some(id) = last_update_id {
                info!("Last id: {}, current id: {}", id, update.id);
                if id >= update.id {
                    return Ok(None);
                }
            }
            self.db.set_last_update_id(message.chat.id(), update.id)?;
            return Ok(self.handle(message).map(|m| (message.chat.clone(), m)));
        }
        Ok(None)
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
