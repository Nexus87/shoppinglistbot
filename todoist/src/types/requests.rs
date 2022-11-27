use serde_json::Error;
use uuid::Uuid;
use serde::Serialize;
use crate::primitives::Integer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetProjectDataRequest {
    pub token: String,
    pub project_id: Integer,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct GetProjectsRequest {
    pub token: String,
    pub sync_token: String,
    pub resource_types: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Task<'a> {
    content: &'a str,
    project_id: Integer,
    date_string: Option<String>,
    date_lang: Option<String>,
    due_date_utc: Option<String>,
    priority: Option<Integer>,
    indent: Option<Integer>,
    item_order: Option<Integer>,
    day_order: Option<Integer>,
    collapsed: Option<Integer>,
    labels: Option<Vec<Integer>>,
    assigned_by_uid: Option<Integer>,
    responsible_uid: Option<Integer>,
    auto_reminder: Option<bool>,
    auto_parse_labels: Option<bool>,
}

impl<'a> Task<'a> {
    pub fn new(content: &'a str, project_id: Integer) -> Self {
        Task {
            content,
            project_id,
            date_string: None,
            date_lang: None,
            due_date_utc: None,
            priority: None,
            indent: None,
            item_order: None,
            day_order: None,
            collapsed: None,
            labels: None,
            assigned_by_uid: None,
            responsible_uid: None,
            auto_reminder: None,
            auto_parse_labels: None
        }
    }
}
#[derive(Serialize, Debug, PartialEq)]
pub struct WriteResource<'a> {
    commands: String,
    token: &'a String,
}

impl<'a> WriteResource<'a> {
    pub fn new<T>(commands: &Vec<Command<T>>, token: &'a String) -> Result<Self, Error> where T:Serialize{
        let commands = serde_json::to_string(commands)?;
        Ok(WriteResource {
            commands,
            token,
        })
    }
}

pub enum CommandType {
    AddTask
}

impl CommandType {
    fn to_string(&self) -> String {
        let res = match self {
            CommandType::AddTask => "item_add"
        };

        res.to_string()
    }
}
#[derive(Serialize, Debug, PartialEq)]
pub struct Command<T> where T: Serialize {
    #[serde(rename = "type")]
    cmd_type: String,
    args: T,
    uuid: Uuid,
    temp_id: Option<Uuid>,
}

impl<T> Command<T> where T: Serialize {
    pub fn new(cmd_type: CommandType, args: T) -> Self {
        Command {
            args,
            cmd_type: cmd_type.to_string(),
            temp_id: Some(Uuid::new_v4()),
            uuid: Uuid::new_v4()
        }
    }

    pub fn new_add_task(args: T) -> Self {
        Self::new(CommandType::AddTask, args)
    }
}