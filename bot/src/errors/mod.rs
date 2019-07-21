use sled;
use std::error::Error;

#[derive(Debug, Fail)]
pub enum ShoppingListBotError {
    #[fail(display = "Error in data storage: {}", error_message)]
    StorageError { error_message: String },
    #[fail(display = "Missing env variables: {}", missing_var)]
    InitError { missing_var: String },
    #[fail(display = "Parsing of {} failed: {}", name, err)]
    ParsingError { name: String, err: String },
    #[fail(display = "Serialization failed: {}", err)]
    SerializationError { err: String },
    #[fail(display = "IO Error: {}", error_message)]
    IOError { error_message: String },
    #[fail(display = "Actix error: {}", error_message)]
    MailboxError { error_message: String },    
    #[fail(display = "Telegram error: {}", error_message)]
    TelegramError { error_message: String },    
    #[fail(display = "Hyper error: {}", error_message)]
    HyperError { error_message: String },
}

impl ShoppingListBotError {
    pub fn new_parsing_error(name: String, err: String) -> Self {
        ShoppingListBotError::ParsingError {
            name,
            err,
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

impl From<std::io::Error> for ShoppingListBotError {
    fn from(err: std::io::Error) -> Self {
        ShoppingListBotError::IOError {
            error_message: err.description().to_string()
        }
    }
}

impl From<actix::MailboxError> for ShoppingListBotError {
    fn from(err: actix::MailboxError) -> Self {
        ShoppingListBotError::MailboxError {
            error_message: format!("{}", err)
        }
    }
}
impl From<telegram_bot::Error> for ShoppingListBotError {
    fn from(err: telegram_bot::Error) -> Self {
        ShoppingListBotError::TelegramError{
            error_message: format!("{}", err)
        }
    }
}
impl From<hyper::error::Error> for ShoppingListBotError {
    fn from(err: hyper::error::Error) -> Self {
        ShoppingListBotError::HyperError{
            error_message: format!("{}", err)
        }
    }
}