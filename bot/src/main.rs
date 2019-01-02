extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate todoist;
use todoist::TodoistApi;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use std::env;
use todoist::shopping_list_api::ShoppingListApi;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let todoist_token = env::var("TODOIST_TOKEN").unwrap();
//    let client_ids: Vec<UserId> = env::var("CLIENT_IDS")
////            .unwrap_or(String::from(""))
////            .split(",")
////            .map(|x| x.parse::<Integer>().unwrap())
////            .map(From::from)
////            .collect();
////    let api = Api::configure(token).build(core.handle()).unwrap();
////
////    let future = api.stream()
////        .filter(|update| {
////            let id = match &update.kind {
////                UpdateKind::Message(m) => Some(m.from.id),
////                _ => None
////            };
////
////            if let Some(ref id) = id {
////                return client_ids.contains(&id)
////            }
////            false
////        })
////        .for_each(|update| {
////        if let UpdateKind::Message(message) = update.kind {
////            if let MessageKind::Text {ref data, ..} = message.kind {
////                println!("<{}>: {}", &message.from.first_name, data);
////
////                api.spawn(message.text_reply(
////                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
////                ));
////            }
////        }
////        Ok(())
////    });
    let api = TodoistApi::new(todoist_token);
    let future = api.get_projects();
    core.run(future).unwrap();
}
