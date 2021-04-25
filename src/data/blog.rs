use crate::*;

#[derive(Debug)]
pub struct BlogItem {
    pub filename: String,
    pub title: String,
    pub description: String,
    pub time: DateTime<FixedOffset>,
}

impl BlogItem {
    pub fn parse_rss(blogs: &str) -> Vec<Self> {
        let ch = rss::Channel::read_from(blogs.as_bytes()).unwrap();
        let mut items = ch.items;
        items.reverse();
        items
            .into_iter()
            .map(|item| {
                let filename = std::path::PathBuf::from(item.link.unwrap_or_default())
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let time =
                    DateTime::parse_from_rfc2822(&item.pub_date.unwrap_or_default()).unwrap();
                Self {
                    filename,
                    title: item.title.unwrap_or_default(),
                    description: item.description.unwrap_or_default(),
                    time,
                }
            })
            .collect()
    }
}
