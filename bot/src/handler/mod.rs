use telegram_bot::types::Message;
use telegram_bot::types::MessageKind;

pub mod message_handler;
pub mod einkaufen_handler;

#[derive(PartialEq, Eq, Hash)]
pub enum Commands {
    Config,
    Einkaufen,
    None
}

pub fn handle_message(message: Message) {
    if let MessageKind::Text{ref data, ..} = message.kind {

    }
}

impl Commands {
    fn as_string(&self) -> &str {
        match self {
            Commands::Config => "/config",
            Commands::Einkaufen => "/einkaufen",
            _ => "None"
        }
    }
}

impl From<&str> for Commands {
    fn from(s: &str) -> Self {
        match s {
            "/config" => Commands::Config,
            "/einkaufen" => Commands::Einkaufen,
            _ => Commands::None
        }
    }
}
pub trait CommandHandler{
    fn handle_message(&self, cmd_args: &String);
}