pub mod message_handler;
mod einkaufen_handler;


pub trait CommandHandler: Sync + Send{
    fn handle_message(&self, cmd_args: &str);
}