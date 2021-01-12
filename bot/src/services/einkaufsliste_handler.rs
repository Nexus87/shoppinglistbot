use std::sync::Arc;

use crate::errors::ShoppingListBotError;
use todoist::shopping_list_api::TodoistApi;

pub struct EinkauflisteCommandHandler {
    api: Arc<TodoistApi>,
    project_id: i64,
}

impl EinkauflisteCommandHandler {
    pub fn new(api: Arc<TodoistApi>, project_id: i64) -> Self {
        EinkauflisteCommandHandler { api, project_id }
    }

    pub async fn handle_message(&self) -> Result<Option<String>, ShoppingListBotError> {
        info!("Handle command /einkaufsliste");
        let message = self
            .api
            .get_tasks(self.project_id)
            .await?
            .iter()
            .map(|i| i.content.as_str())
            .collect::<Vec<&str>>()
            .join("\n");
        Ok(Some(message))
    }
}