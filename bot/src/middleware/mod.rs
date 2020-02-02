use std::io;
use std::sync::Arc;

use failure::_core::str::from_utf8;
use futures::{future, Future};
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError, Response};
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{FromState, State, StateData};
use hyper::{Body, StatusCode};
use telegram_bot::{Message, Update, UpdateKind};

use crate::errors::ShoppingListBotError;
use crate::storage::Storage;
use gotham::helpers::http::response::create_empty_response;

pub struct TelegramHandlerMiddlewareDate {
    pub message: Message,
}

impl StateData for TelegramHandlerMiddlewareDate {}

#[derive(Clone)]
pub struct TelegramHandlerMiddleware {
    db: Arc<dyn Storage>,
}

fn extract_json<T>(state: &mut State) -> impl Future<Output = Result<T, ShoppingListBotError>>
where
    T: serde::de::DeserializeOwned,
{
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

fn handle_update(
    db: Arc<dyn Storage>,
    update: Update,
) -> Result<Option<Message>, ShoppingListBotError> {
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

impl TelegramHandlerMiddleware {
    pub fn new(db: &Arc<dyn Storage>) -> Self {
        TelegramHandlerMiddleware { db: db.clone() }
    }

    async fn call_impl<Chain>(
        self,
        mut state: State,
        chain: Chain,
    ) -> Result<(State, Response<Body>), (State, HandlerError)>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + Send + 'static,
        Self: Sized,
    {
        let update = extract_json::<Update>(&mut state).await.map_err(|err| (state, err))?;
        let message = handle_update(self.db, update);

        let res = match message {
            Ok(Some(message)) => {
                state.put(TelegramHandlerMiddlewareDate { message });
                info!("Handling message");
                chain(state)
            }
            Ok(None) => {
                info!("No relevant message");
                let res = create_empty_response(&state, StatusCode::OK);
                (state, res)
            }
            Err(e) => {
                info!("Error!");
                let err = (state, e.into_handler_error());
                err
            }
        };
    }
}
impl NewMiddleware for TelegramHandlerMiddleware {
    type Instance = TelegramHandlerMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(TelegramHandlerMiddleware {
            db: self.db.clone(),
        })
    }
}

impl Middleware for TelegramHandlerMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + Send + 'static,
        Self: Sized,
    {
        let res = async {
            let update = extract_json::<Update>(&mut state).await?;
            let message = handle_update(self.db, update);

            match message {
                Ok(Some(message)) => {
                    state.put(TelegramHandlerMiddlewareDate { message });
                    info!("Handling message");
                    chain(state)
                }
                Ok(None) => {
                    info!("No relevant message");
                    let res = create_empty_response(&state, StatusCode::OK);
                    Box::new(future::ok((state, res)))
                }
                Err(e) => {
                    info!("Error!");
                    let err = (state, e.into_handler_error());
                    Box::new(future::err(err))
                }
            }
        };
        res
    }
}
