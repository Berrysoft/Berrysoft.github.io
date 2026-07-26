use crate::{data::*, *};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Repo {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Payload {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub r#ref: String,
    #[serde(default)]
    pub head: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub repo: Repo,
    #[serde(default)]
    pub payload: Payload,
    pub created_at: DateTime<Utc>,
}
