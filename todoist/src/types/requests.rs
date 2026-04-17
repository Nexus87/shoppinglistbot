use crate::types::primitives::Integer;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;
use uuid::Uuid;

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
pub struct GetDataRequest {
    pub token: String,
    pub project_id: Integer,
}
#[derive(Serialize, Debug, PartialEq)]
pub struct TaskId<'a> {
    pub id: &'a str,
}
impl<'a> TaskId<'a> {
    pub fn new(id: &'a str) -> Self {
        TaskId { id }
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Task<'a> {
    content: &'a str,
    project_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date_utc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indent: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_order: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day_order: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collapsed: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<Integer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assigned_by_uid: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    responsible_uid: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_reminder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_parse_labels: Option<bool>,
}
impl<'a> Task<'a> {
    pub fn new(content: &'a str, project_id: &'a str) -> Self {
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
            auto_parse_labels: None,
        }
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct WriteResource<'a> {
    commands: String,
    token: &'a String,
}

impl<'a> WriteResource<'a> {
    pub fn new<T>(commands: &Vec<Command<T>>, token: &'a String) -> Result<Self, Error>
    where
        T: Serialize,
    {
        let commands = serde_json::to_string(commands)?;
        Ok(WriteResource { commands, token })
    }
}

pub enum CommandType {
    AddTask,
    DeleteTask
}

impl CommandType {
    fn to_string(&self) -> String {
        let res = match self {
            CommandType::AddTask => "item_add",
            CommandType::DeleteTask => "item_delete"
        };

        res.to_string()
    }
}
#[derive(Serialize, Debug, PartialEq)]
pub struct CommandRequest
{
    commands: String,
}

impl CommandRequest
{
    pub fn new<T>(command: &Vec<Command<T>>) -> Result<Self, Error>
    where
        T: Serialize
    {
        let body = serde_json::to_string(command)?;
        Ok(CommandRequest { commands: body })
    }
}
#[derive(Serialize, Debug, PartialEq)]
pub struct Command<T>
where
    T: Serialize,
{
    #[serde(rename = "type")]
    cmd_type: String,
    args: T,
    uuid: Uuid,
    temp_id: Option<Uuid>,
}

impl<T> Command<T>
where
    T: Serialize,
{
    pub fn new(cmd_type: CommandType, args: T) -> Self {
        Command {
            args,
            cmd_type: cmd_type.to_string(),
            temp_id: Some(Uuid::new_v4()),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn new_add_task(args: T) -> Self {
        Self::new(CommandType::AddTask, args)
    }
    
    pub fn delete_task(args: T) -> Self {
        Self::new(CommandType::DeleteTask, args)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SyncResponse {
    sync_status: HashMap<String, String>,
}
