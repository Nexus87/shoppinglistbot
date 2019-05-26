use errors::ShoppingListBotError;
use storage::SledStorage;
use todoist::TodoistApi;
use actix::prelude::*;

pub struct StoreCommandHandler {
}


impl StoreCommandHandler{


    pub fn handle_message_store(&self, cmd_args: &str) -> Option<String> {
        info!("Handle command /store");
        self.storage.set_temp("myKey", cmd_args.to_string()).unwrap();
        None
    }
    pub fn handle_message_load(&self) -> Option<String> {

    }
}

pub struct Einkaufen {
    args: String
}

impl Message for Einkaufen {
    type Result = ();
}

// pub struct Load {}
// pub struct Store {
//     text: String
// }

// impl Message for Load {
//     type Result = Result<String, ShoppingListBotError>;
// }

// impl Message for Store {
//     type Result = Result<(), ShoppingListBotError>;
// }

pub struct CommandActor {
    api: TodoistApi,
    project_id: i64,
}

impl Actor for CommandActor {
    type Context = SyncContext<Self>;
}

impl Handler<Einkaufen> for CommandActor {
    type Result = ();

    fn handle(&mut self, msg: Einkaufen, ctx: &mut SyncContext<Self>) -> Self::Result {
        info!("Handle command /einkaufen");
        let items = split_args(msg.args.as_str());
        info!("With args {:?}", items);
        let future = self.api.add_tasks(&items, self.project_id);
        // let mut runtime = Runtime::new().expect("failed to start new Runtime");
        // runtime
        //     .block_on(future)
        //     .expect("shutdown cannot error");

        if items.len() > 0
        { Some(format!("Added {} items", items.len())); } else { Some("Nothing to add".to_string()); }
    }
}

// impl Handler<Store> for CommandActor {
//     type Result = Result<(), ShoppingListBotError>;
//     fn handle(&mut self, msg: Store, ctx: &mut SyncContext<Self>) -> Self::Result {
//         info!("Handle command /load");
//         self.storage.send(super::storage::).get_temp("myKey")?
//     }
// }
impl CommandActor {
    pub fn new(api: TodoistApi, project_id: i64) -> Self {
        CommandActor {
            api,
            project_id,
        }
    }
}

fn split_args(cmd_args: &str) -> Vec<&str> {
    cmd_args.split(';')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect()
}


#[test]
fn split_args_test() {
    let args = "1; 4; 5 ; 6; ";
    let items = split_args(args);
    let expected = vec![
        String::from("1"),
        String::from("4"),
        String::from("5"),
        String::from("6")
    ];
    assert_eq!(items, expected);
}