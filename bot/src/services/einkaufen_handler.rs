use todoist::shopping_list_api::TodoistApi;
use todoist::shopping_list_api::ShoppingListApi;
use tokio::runtime::Runtime;
use std::sync::Arc;

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
    
    pub fn handle_message(&self, cmd_args: &str) -> Option<String> {
        info!("Handle command /einkaufen");
        let items = split_args(cmd_args);
        info!("With args {:?}", items);
        let future = self.api.add_tasks(&items, self.project_id);
        let mut runtime = Runtime::new().expect("failed to start new Runtime");
        runtime
            .block_on(future)
            .expect("shutdown cannot error");
        
        if items.len() > 0 
            {Some(format!("Added {} items", items.len() )) }
            else {Some("Nothing to add".to_string())} 
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