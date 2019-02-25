use sled;
#[derive(Debug, Fail)]
pub enum ShoppingListBotError {
    #[fail(display = "Error in data storage: {}", error_message)]
    StorageError {
        error_message: String
    }
}

impl From<sled::Error<()>> for ShoppingListBotError {
    fn from(err: sled::Error<()>) -> ShoppingListBotError {
        ShoppingListBotError::StorageError{
            error_message: format!("{}", err)
        }
    }
}