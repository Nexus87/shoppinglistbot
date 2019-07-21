use todoist::shopping_list_api::TodoistApi;
use todoist::shopping_list_api::ShoppingListApi;
use tokio::runtime::Runtime;
use actix::{Actor, Handler, Context};
use actix::Message;
use errors::ShoppingListBotError;

pub struct Einkaufen {
    pub args: String
}

impl Message for Einkaufen {
    type Result = Result<(), ShoppingListBotError>;
}

pub struct EinkaufenActor {
    api: TodoistApi,
    project_id: i64,
}

impl Actor for EinkaufenActor {
    type Context = Context<Self>;
}

impl Handler<Einkaufen> for EinkaufenActor {
    type Result = Result<(), ShoppingListBotError>;

    fn handle(&mut self, msg: Einkaufen, _: &mut Context<Self>) -> Self::Result {
        info!("Handle command /einkaufen");
        let items = split_args(msg.args.as_str());
        info!("With args {:?}", items);
        let future = self.api.add_tasks(&items, self.project_id);
        let mut runtime = Runtime::new().expect("failed to start new Runtime");
        runtime
            .block_on(future)
            .expect("shutdown cannot error");

        if items.len() > 0
        { Some(format!("Added {} items", items.len())); } else { Some("Nothing to add".to_string()); }
        Ok(())
    }
}

impl EinkaufenActor {
    pub fn new(api: TodoistApi, project_id: i64) -> Self {
        EinkaufenActor {
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