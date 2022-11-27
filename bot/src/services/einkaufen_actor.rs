use todoist::shopping_list_api::TodoistApi;
use todoist::shopping_list_api::ShoppingListApi;
use tokio::runtime::Runtime;
use crate::errors::ShoppingListBotError;

pub struct Einkaufen {
    pub args: String
}

pub struct EinkaufenActor {
    api: TodoistApi,
    project_id: i64,
}

impl EinkaufenActor {

    async fn handleEinkaufen(&mut self, msg: Einkaufen) -> Result<String, ShoppingListBotError> {
        info!("Handle command /einkaufen");
        let items = split_args(msg.args.as_str());
        info!("With args {:?}", items);
        self.api.add_tasks(&items, self.project_id)
            .await?;

        let message = if items.len() > 0
        {
            format!("Added {} items", items.len())
        } else {
            "Nothing to add".to_string()
        };
        
        Ok(message)
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