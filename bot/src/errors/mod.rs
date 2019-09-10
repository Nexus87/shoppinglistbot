use sled;
use gotham::handler::{IntoHandlerError, HandlerError};
use std::error::Error;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum ShoppingListBotError {
    #[fail(display = "Error in data storage: {}", error_message)]
    StorageError { error_message: String },
    #[fail(display = "Missing env variables: {}", missings_var)]
    InitError { missings_var: String },
    #[fail(display = "Parsing of {} failed: {}", name, err)]
    ParsingError { name: String, err: String },
    #[fail(display = "Serialization failed: {}", err)]
    SerializationError {err: String},
    #[fail(display = "Hyper failed: {}", err)]
    HyperError {err: String},
    #[fail(display = "Telegram failed: {}", err)]
    TelegramError {err: String}
}

impl ShoppingListBotError {
    pub fn new_parsing_error(name: String, err: String) -> Self {
        ShoppingListBotError::ParsingError {
            name,
            err
        }
    }
}

impl From<sled::Error> for ShoppingListBotError {
    fn from(err: sled::Error) -> ShoppingListBotError {
        ShoppingListBotError::StorageError {
            error_message: format!("{}", err),
        }
    }
}
impl From<bincode::Error> for ShoppingListBotError {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        ShoppingListBotError::SerializationError {
            err: err.description().to_string()
        }
    }
}
impl From<hyper::Error> for ShoppingListBotError {
    fn from(err: hyper::Error) -> Self {
        ShoppingListBotError::HyperError {
            err: err.description().to_string()
        }
    }
}

impl From<telegram_bot::Error> for ShoppingListBotError {
    fn from(err: telegram_bot::Error) -> Self {
        ShoppingListBotError::TelegramError {
            err: err.description().to_string()
        }
    }
}

impl IntoHandlerError for ShoppingListBotError {
    fn into_handler_error(self) -> HandlerError {
        trace!(" converting Error to HandlerError: {}", self);
        self.compat().into_handler_error()
    }
}