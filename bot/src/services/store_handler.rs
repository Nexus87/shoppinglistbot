use storage::Storage;
use std::sync::Arc;
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

    pub fn handle_message_store(&self, cmd_args: &str) -> Option<String> {
        info!("Handle command /store");
        self.storage.set_temp("myKey", cmd_args.to_string()).unwrap();
        None
    }
    pub fn handle_message_load(&self) -> Option<String> {
        info!("Handle command /load");
        self.storage.get_temp("myKey").unwrap()
    }

}