use serde::*;

pub mod github;

#[derive(Debug, Clone, Deserialize)]
pub struct PersonalProject {
    pub name: String,
    pub url: String,
    pub language: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendLink {
    pub name: String,
    pub title: String,
    pub url: String,
}
