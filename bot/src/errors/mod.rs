use std::string::FromUtf8Error;
use sled;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShoppingListBotError {
    #[error("Error in data storage: {error_message:?}")]
    StorageError { error_message: String },
    #[error("Missing env variables: {missings_var:?}")]
    InitError { missings_var: String },
    #[error("Parsing of {name:?} failed: {err:?}")]
    ParsingError { name: String, err: String },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("IO operation failed: {err:?}")]
    IoError { err: String },
    #[error(transparent)]
    Other(#[from] FromUtf8Error),
}

impl ShoppingListBotError {
    pub fn new_parsing_error(name: String, err: String) -> Self {
        ShoppingListBotError::ParsingError { name, err }
    }
}

impl From<sled::Error> for ShoppingListBotError {
    fn from(err: sled::Error) -> ShoppingListBotError {
        ShoppingListBotError::StorageError {
            error_message: format!("{}", err),
        }
    }
}

impl From<rocksdb::Error> for ShoppingListBotError {
    fn from(err: rocksdb::Error) -> ShoppingListBotError {
        ShoppingListBotError::StorageError {
            error_message: format!("{}", err),
        }
    }
}

impl From<std::io::Error> for ShoppingListBotError {
    fn from(err: std::io::Error) -> Self {
        ShoppingListBotError::IoError {
            err: err.to_string(),
        }
    }
}
