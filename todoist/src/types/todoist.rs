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
pub struct GetProjectData {
    project: Project,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    is_deleted: Integer,
    date_string: String,
    date_added: String,
    item_order: Integer,
    due_date: Option<DueDate>,
    content: String,
    id: Integer,
    user_id: Integer,
    date_lang: String,
    assigned_by_uid: Integer,
    in_history: Integer,
    is_archived: Integer,
    project_id: Integer,
    collapsed: Integer,
    indent: Integer,
    checked: Integer,
    priority: Integer,
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