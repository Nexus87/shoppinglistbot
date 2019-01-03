use todoist::shopping_list_api::TodoistApi;
use handler::CommandHandler;

pub struct EinkaufenCommandHandler {
    api: TodoistApi
}


impl EinkaufenCommandHandler {
    pub fn new(api: TodoistApi) -> Self {
        EinkaufenCommandHandler {
            api
        }
    }
}
impl CommandHandler for EinkaufenCommandHandler {
    fn handle_message(&self, cmd_args: &String) {
        let items: Vec<&str> = cmd_args.split(";").collect();
        for item in items {
            println!("{:?}", item)
        };
    }
}