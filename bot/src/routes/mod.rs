use failure::_core::str::from_utf8;
use futures::future;
use futures::prelude::*;
use gotham::error::Result;
use gotham::handler::{Handler, HandlerError, HandlerFuture, IntoHandlerError, NewHandler};
use gotham::helpers::http::response::create_empty_response;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::{Body, StatusCode};
use telegram_bot::Update;

use services::{ShoppingBotService, TelegramMessageSendService};
use errors::ShoppingListBotError;

#[derive(Clone)]
pub struct TelegramWebhook {
    telegram_service: TelegramMessageSendService,
    shopping_bot_service: ShoppingBotService,
}

impl Handler for TelegramWebhook {
    fn handle(self, mut state: State) -> Box<HandlerFuture> {
        let s = self.clone();
        let result = extract_json::<Update>(&mut state)
            .and_then(move |payload| {
                self.shopping_bot_service.handle_message(payload)
            })
            .and_then(move |(c, m)| s.telegram_service.send_message(c, &m))
            .then(move |r| {
                let res = match r {
                    Err(e) => {
                        error!("{}", e);
                        create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
                    }

                    Ok(_) => {
                        create_empty_response(&state, StatusCode::OK)
                    }
//                    _ => create_empty_response(&state, StatusCode::OK)
                };

                future::ok((state, res))
            });
        Box::new(result)
    }
}

impl NewHandler for TelegramWebhook {
    type Instance = Self;

    fn new_handler(&self) -> Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl TelegramWebhook {
    pub fn new(telegram_service: TelegramMessageSendService, send_message_service: ShoppingBotService) -> Self {
        TelegramWebhook { telegram_service, shopping_bot_service: send_message_service }
    }
}
//#[post("/webhook", format = "json", data = "<payload>")]
//fn telegram_webhook(payload: Json<Update>, telegram_service: State<Box<dyn TelegramMessageService>>,
//                    message_send_service: State<Box<dyn MessageSendService>>) -> Result<(), ()> {
//    match telegram_service.handle_message(&payload) {
//        Err(e) => error!("{}", e),
//        Ok(Some((c, m))) => message_send_service.send_message(c, &m),
//        _ => ()
//    }
//
//    Ok(())
//}

fn extract_json<T>(state: &mut State) -> impl Future<Item=T, Error=ShoppingListBotError>
    where T: serde::de::DeserializeOwned {
    Body::take_from(state)
        .concat2()
        .and_then(|body| {
            let b = body.to_vec();
            let result = from_utf8(&b).unwrap();
            future::ok(result.to_string())
        })
        .and_then(|v| {
            let result = serde_json::from_str::<T>(&v).unwrap();
            future::ok(result)
        })
        .map_err(|e| e.into())
}

pub fn get_routes(telegram_service: TelegramMessageSendService, send_message_service: ShoppingBotService) -> Router {
    build_simple_router(move |route| {
        route
            .post("/webhook")
            .to_new_handler(TelegramWebhook::new(telegram_service, send_message_service))
    })
}
