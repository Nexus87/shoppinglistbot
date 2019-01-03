use std::collections::HashMap;
use handler::CommandHandler;
use handler::Commands;
use telegram_bot::types::Message;
use telegram_bot::types::MessageKind;

pub struct MessageHandlerBuilder {
    handler: HashMap<Commands, Box<CommandHandler>>
}
pub struct MessageHandler {
    handler: HashMap<Commands, Box<CommandHandler>>
}

impl MessageHandler {
    pub fn build() -> MessageHandlerBuilder {
        {
            MessageHandlerBuilder {
                handler: HashMap::new()
            }
        }
    }

    pub fn handle(&self, message: Message) {
        if let MessageKind::Text{ref data, ..} = message.kind {
            let split: Vec<&str> = data.split_whitespace().collect();
            if let Some(&cmd) = split.get(0).filter(|x| x.starts_with('/')) {
                let cmd = Commands::from(cmd);
                if let Some(handler) = self.handler.get(&cmd) {
                    let cmd_args = split[1..].join(" ");
                    handler.handle_message(&cmd_args);
                }
            }
        }
    }
}

impl MessageHandlerBuilder {
    pub fn add_handler(mut self, command: Commands, handler: Box<CommandHandler>) -> Self {
        self.handler.insert(command, handler);
        self
    }

    pub fn build(self) -> MessageHandler {
        MessageHandler {
            handler: self.handler
        }
    }
}