use crate::{data::*, *};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Repo {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Commit {
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Payload {
    #[serde(default)]
    pub r#ref: String,
    #[serde(default)]
    pub commits: Vec<Commit>,
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
