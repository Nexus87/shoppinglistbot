use super::einkaufen_handler::EinkaufenCommandHandler;
use handler::CommandHandler;
use telegram_bot::types::Message;
use telegram_bot::types::MessageKind;
use todoist::shopping_list_api::TodoistApi;

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

pub struct MessageHandler {
    einkaufen_handler: EinkaufenCommandHandler,
}

impl MessageHandler {
    pub fn new(token: String, project_id: i64) -> Self {
        let api = TodoistApi::new(token);
        MessageHandler {
            einkaufen_handler: EinkaufenCommandHandler::new(api, project_id),
        }
    }

    pub fn handle(&mut self, message: &Message) {
        if let Some((command, args)) = parse_message(&message.kind) {
            if let Command::Einkaufen = command {
                self.einkaufen_handler.handle_message(&args)
            }
        }
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
    none: ("bla bla", (Command::None, "bla bla")),
}
