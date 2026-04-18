use super::{
    einkaufen_handler::EinkaufenCommandHandler, einkaufsliste_handler::EinkauflisteCommandHandler,
};
use crate::errors::ShoppingListBotError;
use crate::services::TelegramMessageService;
use crate::services::store_handler::StoreCommandHandler;
use crate::storage::Storage;
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;
use rstest::rstest;
use teloxide::prelude::*;
use teloxide::types::{Chat, MediaKind, MediaText, MessageCommon, MessageKind, UpdateKind};
use todoist::shopping_list_api::TodoistApi;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Command {
    Config,
    Einkaufen,
    TestStore,
    TestGet,
    Einkaufsliste,
    None,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "/config" => Command::Config,
            "/einkaufen" => Command::Einkaufen,
            "/store" => Command::TestStore,
            "/load" => Command::TestGet,
            "/einkaufsliste" => Command::Einkaufsliste,
            _ => Command::None,
        }
    }
}
static COMMANDS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<command>/[^@]+)(@[^\s]*)?").unwrap());

fn parse_message(message_kind: &MessageKind) -> Option<(Command, String)> {
    if let MessageKind::Common(MessageCommon {
                                   media_kind: MediaKind::Text(data),
                                   ..
                               }) = message_kind
    {
        let split: Vec<&str> = data.text.splitn(2, ' ').collect();
        let command = match COMMANDS.captures(split.get(0)?) {
            None => Command::None,
            Some(capture) => Command::from(&capture["command"]),
        };
        let args = match command {
            Command::None => data.text.clone(),
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
    einkaufliste_handler: Arc<EinkauflisteCommandHandler>,
    store_handler: Arc<StoreCommandHandler>,
    db: Arc<dyn Storage>,
}

impl ShoppingBotMessageService {
    pub fn new(
        token: String,
        project_id: &String,
        client_ids: Vec<UserId>,
        db: Arc<dyn Storage>,
    ) -> Self {
        let api = Arc::new(TodoistApi::new(token));
        ShoppingBotMessageService {
            client_ids,
            einkaufen_handler: Arc::new(EinkaufenCommandHandler::new(api.clone(), project_id)),
            einkaufliste_handler: Arc::new(EinkauflisteCommandHandler::new(
                api.clone(),
                project_id,
            )),
            db: db.clone(),
            store_handler: Arc::new(StoreCommandHandler::new(db)),
        }
    }

    pub async fn handle(&self, message: &Message) -> Option<String> {
        if let Some(from) = &message.from
            && !self.client_ids.contains(&from.id)
        {
            warn!("Unknown client: {:?}", message.from);
            return None;
        } else if message.from.is_none() {
            return None;
        }

        if let Some((command, args)) = parse_message(&message.kind) {
            return match command {
                Command::Einkaufen => self.einkaufen_handler.handle_message(&args).await.unwrap(),
                Command::Einkaufsliste => self.einkaufliste_handler.handle_message().await.unwrap(),
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
    async fn handle_message(
        &self,
        update: &Update,
    ) -> Result<Option<(Chat, String)>, ShoppingListBotError> {
        let update = update.clone();
        let api = self.clone();

        if let UpdateKind::Message(message) = update.kind {
            let last_update_id = api.db.get_last_update_id(message.chat.id)?;
            if let Some(id) = last_update_id {
                info!("Last id: {}, current id: {}", id.0, update.id.0);
                if id >= update.id {
                    return Ok(None);
                }
            }
            api.db.set_last_update_id(message.chat.id, update.id)?;
            return Ok(api
                .handle(&message)
                .await
                .map(|m| (message.chat.clone(), m)));
        }
        Ok(None)
    }
}

#[rstest]
#[case("/einkaufen bla bla", Command::Einkaufen, "bla bla")]
#[case("/einkaufen", Command::Einkaufen, "")]
#[case("/einkaufen@foo", Command::Einkaufen, "")]
#[case("/config bla bla", Command::Config, "bla bla")]
#[case("/config", Command::Config, "")]
#[case("bla bla", Command::None, "bla bla")]
#[case("einkaufen bla bla", Command::None, "einkaufen bla bla")]
fn parsing_works(#[case] input: &'static str, #[case] expected_command: Command, #[case] expected_args: &str) {
    fn to_message(s: &'static str) -> MessageKind {
        MessageKind::Common(MessageCommon {
            author_signature: None,
            paid_star_count: None,
            effect_id: None,
            forward_origin: None,
            reply_to_message: None,
            external_reply: None,
            quote: None,
            reply_to_story: None,
            sender_boost_count: None,
            edit_date: None,
            media_kind: MediaKind::Text( MediaText {
                text: s.to_string(),
                entities: vec![],
                link_preview_options: None,
            } ),
            reply_markup: None,
            is_automatic_forward: false,
            has_protected_content: false,
            is_from_offline: false,
            business_connection_id: None,
        })
    }
    let expected_args = expected_args.to_string();
    let input = to_message(input);
    let (command, args) = parse_message(&input).unwrap();

    assert_eq!(expected_command, command);
    assert_eq!(expected_args, args);
}
