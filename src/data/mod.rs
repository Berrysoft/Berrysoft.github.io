use serde::*;

mod blog;
mod fetch;
mod github;

pub use blog::*;
pub use fetch::*;
pub use github::Event as GitHubEvent;

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

#[derive(Debug, Clone, Deserialize)]
pub struct Library {
    pub name: String,
    pub url: String,
    pub license: String,
    #[serde(default, rename = "licenseUrl")]
    pub license_url: Option<String>,
}
