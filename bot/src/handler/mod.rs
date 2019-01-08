pub mod message_handler;
pub mod einkaufen_handler;

#[derive(PartialEq, Eq, Hash)]
pub enum Commands {
    Config,
    Einkaufen,
    None
}

impl Commands {
    pub fn as_string(&self) -> &str {
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
pub trait CommandHandler: Sync + Send{
    fn handle_message(&mut self, cmd_args: &String);
}