use std::io;
use std::sync::Arc;

use failure::_core::str::from_utf8;
use futures::{Future, future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{FromState, State, StateData};
use hyper::{Body, StatusCode};
use telegram_bot::{Message, Update, UpdateKind};

use storage::Storage;
use gotham::helpers::http::response::create_empty_response;
use errors::ShoppingListBotError;

pub struct TelegramHandlerMiddlewareDate {
    pub message: Message
}

impl StateData for TelegramHandlerMiddlewareDate {}

#[derive(Clone)]
pub struct TelegramHandlerMiddleware {
    db: Arc<dyn Storage>
}

impl NewMiddleware for TelegramHandlerMiddleware {
    type Instance = TelegramHandlerMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(TelegramHandlerMiddleware {
            db: self.db.clone()
        })
    }
}


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

fn handle_update(db: Arc<dyn Storage>, update: Update) -> Result<Option<Message>, ShoppingListBotError> {
    if let UpdateKind::Message(message) = update.kind {
        let update_id = update.id;
        let last_update_id = db.get_last_update_id(message.chat.id())?;
        if let Some(id) = last_update_id {
            info!("Last id: {}, current id: {}", id, update_id);
            if id >= update_id {
                return Ok(None);
            }
            db.set_last_update_id(message.chat.id(), update_id)?;
        }
        return Ok(Some(message));
    }
    Ok(None)
}

impl Middleware for TelegramHandlerMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
        where
            Chain: FnOnce(State) -> Box<HandlerFuture> + Send + 'static,
            Self: Sized {
        let res = extract_json::<Update>(&mut state)
            .and_then(|update| handle_update(self.db, update))
            .then(|message| {
                
                match message {
                    Ok(Some(message)) => {
                        state.put(TelegramHandlerMiddlewareDate{
                            message
                        });
                        chain(state)
                    }
                    Ok(None) => {
                        let res = create_empty_response(&state, StatusCode::OK);
                        Box::new(future::ok((state, res)))
                    }
                    Err(e) => {
                        let err = (state, e.into_handler_error());
                        Box::new(future::err(err))
                    }
                }
            });
        Box::new(res)
    }
}