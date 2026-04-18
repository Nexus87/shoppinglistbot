use crate::types::primitives::Integer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetProjectsResponse {
    pub results: Vec<Project>,
    pub next_cursor: Option<String>
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetTasksResponse {
    pub results: Vec<Item>,
    pub next_cursor: Option<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub child_order: Integer,
    pub is_collapsed: bool,
    pub color: String,
    pub id: String,
    pub is_archived: bool,
    pub is_deleted: bool,
    pub name: String,
    pub view_style: String
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub content: String,
    pub id: String,
    pub project_id: String,
    pub priority: Integer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DueDate {
    string: String,
    date: String,
    datetime: Option<String>,
    timezone: Option<String>,
}

// impl Default for Project {
//     fn default() -> Project {
//         Project  {
//             is_archived: 0,
//             color: 0,
//             shared: false,
//             inbox_project: false,
//             id: 0,
//             name: "".to_string(),
//             item_order: 0,
//             indent: 0
//         }
//     }
// }