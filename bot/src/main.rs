extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate todoist;

mod handler;

use todoist::TodoistApi;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use std::env;
use handler::Commands;
use handler::einkaufen_handler::EinkaufenCommandHandler;
use handler::message_handler::MessageHandler;

fn build_handler(todoist_token: String) -> MessageHandler {
    let einkaufen_handler = Box::new(EinkaufenCommandHandler::new(TodoistApi::new(todoist_token)));
    MessageHandler::build()
        .add_handler(Commands::Einkaufen, einkaufen_handler)
        .build()
}
fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let todoist_token = env::var("TODOIST_TOKEN").unwrap();
    let client_ids: Vec<UserId> = env::var("CLIENT_IDS")
            .unwrap_or(String::from(""))
            .split(",")
            .map(|x| x.parse::<Integer>().unwrap())
            .map(From::from)
            .collect();
    let api = Api::configure(token).build(core.handle()).unwrap();
    let message_handler = build_handler(todoist_token);
    let future = api.stream()
        .filter(|update| {
            let id = match &update.kind {
                UpdateKind::Message(m) => Some(m.from.id),
                _ => None
            };

            if let Some(ref id) = id {
                return client_ids.contains(&id)
            }
            false
        })
        .for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            message_handler.handle(message);
        }
        Ok(())
    });
    core.run(future).unwrap();
}
