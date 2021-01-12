use std::sync::Arc;

use todoist::shopping_list_api::TodoistApi;
use crate::errors::ShoppingListBotError;

pub struct EinkaufenCommandHandler {
    api: Arc<TodoistApi>,
    project_id: i64,
}


impl EinkaufenCommandHandler {
    pub fn new(api: Arc<TodoistApi>, project_id: i64) -> Self {
        EinkaufenCommandHandler {
            api,
            project_id,
        }
    }

    pub async fn handle_message(&self, cmd_args: &str) -> Result<Option<String>, ShoppingListBotError> {
        info!("Handle command /einkaufen");
        let items = split_args(cmd_args);
        info!("With args {:?}", items);
        self.api.add_tasks(&items, self.project_id).await?;
        let message = match items.len() {
            0 => None,
            1 => Some(String::from("Added 1 item")),
            _ => Some(format!("Added {} items", items.len()))
        };
        Ok(message)
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