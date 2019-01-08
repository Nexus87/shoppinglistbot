use todoist::shopping_list_api::TodoistApi;
use handler::CommandHandler;
use todoist::shopping_list_api::ShoppingListApi;
use tokio::prelude::future::Future;

pub struct EinkaufenCommandHandler {
    api: TodoistApi,
    project_id: i64
}


impl EinkaufenCommandHandler {
    pub fn new(api: TodoistApi, project_id: i64) -> Self {
        EinkaufenCommandHandler {
            api,
            project_id
        }
    }
}
impl CommandHandler for EinkaufenCommandHandler {
    fn handle_message(&mut self, cmd_args: &String) {
        let items: Vec<&str> = cmd_args.split(";").collect();
        for item in items {
            let future = self.api.add_task(&item.to_string(), self.project_id)
                .map(|_| ())
                .map_err(|_| ());
             tokio::executor::spawn(future);
        };
    }
}