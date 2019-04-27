use storage::Storage;
use std::sync::Arc;

pub struct StoreCommandHandler {
    storage: Arc<dyn Storage>
}


impl StoreCommandHandler{
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        StoreCommandHandler {
            storage
        }
    }

    pub fn handle_message(&self, cmd_args: &str) -> Option<String> {
        match cmd_args { 
            "/store" => {
                info!("Handle command /store");
                self.storage.set_temp("myKey", cmd_args.to_string()).unwrap();
                None
                
            }
            "/load" => {
                info!("Handle command /load");
                self.storage.get_temp("myKey").unwrap()
            }
            _ => None
        }
    }

}