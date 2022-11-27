use crate::errors::ShoppingListBotError;
use super::einkaufen_actor::EinkaufenActor;
use todoist::shopping_list_api::TodoistApi;
use telegram_bot::{
    types::Update,
    types::UserId,
    Message as TelegramMessage,
    UpdateKind,
    MessageKind,
};
use crate::storage::{SledActor, CheckAndUpdate};
use crate::services::einkaufen_actor::Einkaufen;
use futures::future::Either;

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

pub struct HandleCommand {
    pub update: Update
}

impl Message for HandleCommand {
    type Result = Result<(), ShoppingListBotError>;
}
#[derive(Clone)]
pub struct ShoppingBotMessageService {
    client_ids: Vec<UserId>,
    einkaufen_handler: Addr<EinkaufenActor>,
    db: SledActor,
}

fn done() -> impl Future<Item=(), Error=ShoppingListBotError> {
    futures::done(Ok(()))
}
fn empty_response() -> Box<Future<Item=(), Error=ShoppingListBotError>> {
    Box::new(futures::done(Ok(())))
}

impl ShoppingBotMessageService {
    pub fn new(token: String, project_id: i64, client_ids: Vec<UserId>, db: Addr<SledActor>) -> Self {
        let api = TodoistApi::new(token);
        ShoppingBotMessageService {
            client_ids,
            einkaufen_handler: EinkaufenActor::new(api, project_id).start(),
            db,
        }
    }

    fn handle_message(&mut self, message: &TelegramMessage) -> impl Future<Item=(), Error=ShoppingListBotError> {
        if !self.client_ids.contains(&message.from.id) {
            warn!("Unknown client: {:?}", message.from);
            return Either::B(done());
        }
        if let Some((command, args)) = parse_message(&message.kind) {
            info!("Command {:?}", command);
            match command {
                Command::Einkaufen => {
                    let result = self.einkaufen_handler.send(Einkaufen { args })
                        .map(|_| ())
                        .map_err(|e| e.into());
                    return Either::A(result);
                }
//                Command::TestStore => {
//                    self.store_handler.handle_message_store(&args);
//                    return None;
//                }
//                Command::TestGet => return self.store_handler.handle_message_load(),
                _ => {
                    info!("Unknown command {:?}", command);
                    return Either::B(done());
                }
            }
        }
        Either::B(done())
    }

    async fn handle(&mut self, update: Update) -> Result<(), ShoppingListBotError> {
        if let UpdateKind::Message(message) = update.kind {
            let mut self2 = self.clone();
            let result = self.db
                .send(CheckAndUpdate { chat_id: message.chat.id(), update_id: update.id })
                .map_err(|e| e.into())
                .and_then(move |res| {
                    if let Ok(true) = res {
                        return Either::A(self2.handle_message(&message));
                    }
                    Either::B(futures::done(Ok(())))
                });
            return Box::new(result);
        }
        Ok(())
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
