use todoist::shopping_list_api::TodoistApi;
use std::sync::Arc;
use futures::{Future};
use errors::ShoppingListBotError;

#[derive(Clone)]
pub struct EinkaufenCommandHandler {
    api: Arc<TodoistApi>,
    project_id: i64,
}


impl EinkaufenCommandHandler {
    pub fn new(api: TodoistApi, project_id: i64) -> Self {
        EinkaufenCommandHandler {
            api: Arc::new(api),
            project_id,
        }
    }
    
    pub fn handle_message(self, cmd_args: String) -> impl Future<Item=String, Error=ShoppingListBotError> {
        info!("Handle command /einkaufen");
        let items = split_args(cmd_args);
        info!("With args {:?}", items);
        
        self.api.add_tasks(items, self.project_id)
            .map(|ret| {
                if ret.len() > 0
                {format!("Added {} items", ret.len() ) }
                else {"Nothing to add".to_string()}        
            })
            .map_err(|err| err.into())
         
    }
    
}

fn split_args(cmd_args: String) -> Vec<String> {
    cmd_args.split(';')
        .map(str::trim)
        .map(|x| x.to_string())
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