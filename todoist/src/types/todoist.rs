use crate::types::primitives::Integer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetProjectsResponse {
    pub projects: Vec<Project>,
    pub full_sync: bool,
    pub sync_token: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub is_archived: Integer,
    pub color: Integer,
    pub shared: bool,
    #[serde(default)]
    pub inbox_project: bool,
    pub id: Integer,
    pub name: String,
    pub item_order: Integer,
    pub indent: Integer,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub order: Integer,
    pub content: String,
    pub id: Integer,
    pub project_id: Integer,
    pub priority: Integer,
    pub assigner: Integer
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DueDate {
    string: String,
    date: String,
    datetime: Option<String>,
    timezone: Option<String>,
}

impl Default for Project {
    fn default() -> Project {
        Project  {
            is_archived: 0,
            color: 0,
            shared: false,
            inbox_project: false,
            id: 0,
            name: "".to_string(),
            item_order: 0,
            indent: 0
        }
    }
}