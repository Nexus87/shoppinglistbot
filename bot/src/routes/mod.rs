use std::sync::Arc;

use futures::future;
use futures::prelude::*;
use gotham::error::Result;
use gotham::handler::{Handler, HandlerFuture, NewHandler};
use gotham::helpers::http::response::create_empty_response;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::{build_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::StatusCode;

use crate::middleware::{TelegramHandlerMiddleware, TelegramHandlerMiddlewareDate};
use crate::services::{ShoppingBotService, TelegramMessageSendService};
use crate::storage::Storage;

#[derive(Clone)]
pub struct TelegramWebhook {
    telegram_service: TelegramMessageSendService,
    shopping_bot_service: ShoppingBotService,
}

impl Handler for TelegramWebhook {
    fn handle(self, mut state: State) -> Box<HandlerFuture> {
        let s = self.clone();
        let message = TelegramHandlerMiddlewareDate::take_from(&mut state).message;
        let result = self.shopping_bot_service.handle_message(message)
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

pub fn get_routes(db: &Arc<dyn Storage>, telegram_service: TelegramMessageSendService, send_message_service: ShoppingBotService) -> Router {
    let (chain, pipelines) = single_pipeline(
        new_pipeline()
            .add(TelegramHandlerMiddleware::new(db)
            ).build());
    build_router(chain, pipelines, move |route| {
        route
            .post("/webhook")
            .to_new_handler(TelegramWebhook::new(telegram_service, send_message_service))
    })
}
