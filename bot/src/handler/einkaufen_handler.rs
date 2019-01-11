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

impl CommandHandler for EinkaufenCommandHandler {
    fn handle_message(&mut self, cmd_args: &str) {
        let items: Vec<String> = cmd_args.split(';')
            .map(String::from)
            .collect();
        let future = self.api.add_tasks(&items, self.project_id)
            .map(|_| ())
            .map_err(|_| ());
        tokio::executor::spawn(future);
    }
}