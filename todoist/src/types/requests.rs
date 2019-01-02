use types::primitives::Integer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetProjectDataRequest {
    pub token: String,
    pub project_id: Integer
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetProjectsRequest {
    pub token: String,
    pub sync_token: String,
    pub resource_types: String
}