pub mod message_handler;
mod einkaufen_handler;


pub trait CommandHandler: Sync + Send{
    fn handle_message(&mut self, cmd_args: &str);
}