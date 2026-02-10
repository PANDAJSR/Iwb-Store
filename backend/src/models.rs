use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub name: String,
    pub email: String,
    pub age: Option<u32>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserQuery {
    pub name: Option<String>,
    pub email: Option<String>,
    pub min_age: Option<u32>,
    pub max_age: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub app_name: String,
    pub app_id: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub repository: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}