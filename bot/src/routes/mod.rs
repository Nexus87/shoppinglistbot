use telegram_bot::Update;
use gotham::handler::{Handler, HandlerError, HandlerFuture, IntoHandlerError, NewHandler};
use gotham::state::{State, FromState};
use futures::prelude::*;
use services::{ShoppingBotMessageService, TelegramMessageSendService};
use hyper::{Body, StatusCode};
use failure::_core::str::from_utf8;
use futures::future;
use gotham::error::Result;
use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DrawRoutes, DefineSingleRoute};
use gotham::helpers::http::response::create_empty_response;

pub struct TelegramWebhook {
    telegram_service: TelegramMessageSendService,
    send_message_service: ShoppingBotMessageService,
}

impl Handler for TelegramWebhook {
    fn handle(self, mut state: State) -> Box<HandlerFuture> {
        let result = extract_json::<Update>(&mut state)
            .then(|payload| {
                let res = match self.telegram_service.handle_message(&payload.unwrap()) {
                    Err(e) =>{
                        error!("{}", e);
                        create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR)
                    } 
                        
                    Ok(Some((c, m))) => {
                        self.send_message_service.send_message(c, &m);
                        create_empty_response(&state, StatusCode::OK)
                    }
                    _ => create_empty_response(&state, StatusCode::OK)
                };
                
                future::ok((state, res))
            });
        Box::new(result)
    }
}

impl NewHandler for TelegramWebhook {
    type Instance = Self;
    
    fn new_handler(&self) -> Result<Self::Instance>{
        Ok(self.clone())
    }
}
impl TelegramWebhook {
    pub fn new(telegram_service: TelegramMessageSendService, send_message_service: ShoppingBotMessageService) -> Self {
        TelegramWebhook{telegram_service, send_message_service}
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

fn extract_json<T>(state: &mut State) -> impl Future<Item=T, Error=HandlerError>
    where T: serde::de::DeserializeOwned {
    Body::take_from(state)
        .concat2()
        .and_then(|body| {
            let b = body.to_vec();
            let result = from_utf8(&b).unwrap();
            future::ok(result)
        })
        .and_then(|v| {
            let result = serde_json::from_str::<T>(&v).unwrap();
            future::ok(result)
        })
        .map_err(|e| e.into_handler_error())
}

pub fn get_routes(telegram_service: TelegramMessageSendService, send_message_service: ShoppingBotMessageService) -> Router {
    build_simple_router(move |route| {
        route
            .post("/webhook")
            .to_new_handler(TelegramWebhook::new(telegram_service, send_message_service))
        
    })
}
