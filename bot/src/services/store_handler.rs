use storage::Storage;
use std::sync::Arc;
use futures::Future;
use errors::ShoppingListBotError;

#[derive(Clone)]
pub struct StoreCommandHandler {
    storage: Arc<dyn Storage>
}


impl StoreCommandHandler{
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        StoreCommandHandler {
            storage
        }
    }

    pub fn handle_message_store(&self, cmd_args: &str) -> impl Future<Item=String, Error=ShoppingListBotError> {
        info!("Handle command /store");
        self.storage.set_temp("myKey", cmd_args.to_string()).unwrap();
        futures::empty()
    }
    pub fn handle_message_load(&self) -> impl Future<Item=String, Error=ShoppingListBotError> {
        info!("Handle command /load");
        let  res = self.storage.get_temp("myKey").unwrap().unwrap_or_default();
        futures::future::ok(res)
    }

}