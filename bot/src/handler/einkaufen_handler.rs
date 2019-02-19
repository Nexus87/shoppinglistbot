use todoist::shopping_list_api::TodoistApi;
use handler::CommandHandler;
use todoist::shopping_list_api::ShoppingListApi;
use tokio::prelude::future::Future;

pub struct EinkaufenCommandHandler {
    api: TodoistApi,
    project_id: i64,
}


impl EinkaufenCommandHandler {
    pub fn new(api: TodoistApi, project_id: i64) -> Self {
        EinkaufenCommandHandler {
            api,
            project_id,
        }
    }
}

fn split_args(cmd_args: &str) -> Vec<String> {
    cmd_args.split(';')
        .map(|x| String::from(x.trim()))
        .filter(|x| !x.is_empty())
        .collect()
}

impl CommandHandler for EinkaufenCommandHandler {
    fn handle_message(&self, cmd_args: &str) {
        let items = split_args(cmd_args);
        let future = self.api.add_tasks(&items, self.project_id)
            .map(|_| ())
            .map_err(|_| ());
        tokio::run(future);
    }
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