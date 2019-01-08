extern crate futures;
extern crate telegram_bot;
extern crate todoist;
extern crate tokio;

mod handler;

use futures::Stream;
use telegram_bot::*;
use std::env;
use handler::message_handler::MessageHandler;
use tokio::prelude::future::Future;
use std::result::Result::Ok;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let todoist_token = env::var("TODOIST_TOKEN").unwrap();
    let project_id: i64 = env::var("PROJECT_ID").unwrap().parse().unwrap();

    let client_ids: Vec<UserId> = env::var("CLIENT_IDS")
            .unwrap_or(String::from(""))
            .split(",")
            .map(|x| x.parse::<Integer>().unwrap())
            .map(From::from)
            .collect();

    let api = Api::configure(token).build().unwrap();
    let mut message_handler = MessageHandler::new(todoist_token, project_id);
    let future = api.stream()
        .filter(move |update| {
            let id = match &update.kind {
                UpdateKind::Message(m) => Some(m.from.id),
                _ => None
            };

            if let Some(ref id) = id {
                return client_ids.contains(&id)
            }
            false
        })
        .for_each(move |update| {
        if let UpdateKind::Message(message) = update.kind {
            message_handler.handle(&message);
        }
        Ok(())
    })
        .map_err(|_| ());

    tokio::run(future);
}
