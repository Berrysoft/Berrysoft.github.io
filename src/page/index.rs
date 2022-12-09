use crate::{data::*, layout::*, *};

pub struct IndexPage {
    projects: JsonFetcher<PersonalProject, PersonalProjectWrapper>,
    github_events: JsonFetcher<GitHubEvent, GitHubEventWrapper>,
    links: JsonFetcher<FriendLink, FriendLink>,
}

pub enum IndexPageMessage {
    Projects(JsonFetcherMessage<PersonalProject, PersonalProjectWrapper>),
    GitHubEvents(JsonFetcherMessage<GitHubEvent, GitHubEventWrapper>),
    FriendLinks(JsonFetcherMessage<FriendLink, FriendLink>),
}

impl Component for IndexPage {
    type Message = IndexPageMessage;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            projects: JsonFetcher::new("/data/projects.json", ctx, IndexPageMessage::Projects),
            github_events: JsonFetcher::new(
                "//api.github.com/users/berrysoft/events",
                ctx,
                IndexPageMessage::GitHubEvents,
            ),
            links: JsonFetcher::new("/data/links.json", ctx, IndexPageMessage::FriendLinks),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexPageMessage::Projects(msg) => {
                self.projects.update(msg);
                true
            }
            IndexPageMessage::GitHubEvents(mut msg) => {
                if let Ok(events) = msg {
                    msg = Ok(events
                        .into_iter()
                        .filter(|e| e.r#type == "PushEvent")
                        .collect());
                }
                self.github_events.update(msg);
                true
            }
            IndexPageMessage::FriendLinks(msg) => {
                self.links.update(msg);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let projects = self
            .projects
            .get()
            .map(|projects| {
                html! {
                    <DataGrid<PersonalProjectWrapper> data={projects}>
                        <DataGridColumn<PersonalProjectWrapper> header="名称" prop={PersonalProjectProperties::Name} sortable=true/>
                        <DataGridColumn<PersonalProjectWrapper> header="主要语言" prop={PersonalProjectProperties::Language} sortable=true/>
                        <DataGridColumn<PersonalProjectWrapper> header="简介" prop={PersonalProjectProperties::Description}/>
                    </DataGrid<PersonalProjectWrapper>>
                }
            })
            .unwrap_or_default();
        let github_events_node = self
            .github_events
            .get()
            .map(|events| {
                html! {
                    <DataGrid<GitHubEventWrapper> data={events}>
                        <DataGridColumn<GitHubEventWrapper> header="消息" prop={GitHubEventProperties::Message}/>
                        <DataGridColumn<GitHubEventWrapper> header="时间" prop={GitHubEventProperties::Time}/>
                        <DataGridColumn<GitHubEventWrapper> header="存储库" prop={GitHubEventProperties::Repo}/>
                        <DataGridColumn<GitHubEventWrapper> header="引用" prop={GitHubEventProperties::Ref}/>
                    </DataGrid<GitHubEventWrapper>>
                }
            })
            .unwrap_or_default();
        let friend_links = self
            .links
            .get()
            .map(|links| {
                links.iter().map(|link| html! {
                    <a class="list-group-item list-group-item-action" href={link.url.clone()} target="_blank">
                        {&format!("{} - {}", link.name, link.title)}
                    </a>
                }).collect::<Vec<Html>>()
            })
            .unwrap_or_default();
        html! {
            <div class="container">
                <div class="fade-in fade-in-1">
                    <h1>{"Berrysoft 的 HTML 实验室"}</h1>
                    <p>
                        {"本网站使用 "}
                        <a href="https://www.rust-lang.org/zh-CN/" target="_blank">{"Rust"}</a>
                        {" 开发，有各种各样的实验项目。"}
                    </p>
                </div>
                <div class="fade-in fade-in-2">
                    <h2>{"其它个人开源项目"}</h2>
                    <div class="table-responsive-xl">{projects}</div>
                </div>
                <div class="fade-in fade-in-3">
                    <h2>{"GitHub 事件"}</h2>
                    <div class="table-responsive-xl">{github_events_node}</div>
                </div>
                <div class="fade-in fade-in-4">
                    <h2>{"友情链接"}</h2>
                    <div class="list-group list-group-flush">{friend_links}</div>
                </div>
            </div>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PersonalProjectWrapper {
    name: PersonalProjectName,
    language: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonalProjectProperties {
    Name,
    Language,
    Description,
}

impl From<PersonalProject> for PersonalProjectWrapper {
    fn from(proj: PersonalProject) -> Self {
        Self {
            name: PersonalProjectName {
                name: proj.name,
                url: proj.url,
            },
            language: proj.language,
            description: proj.description,
        }
    }
}

impl DataGridItem for PersonalProjectWrapper {
    type Prop = PersonalProjectProperties;

    fn prop(&self, p: &Self::Prop) -> &dyn DataGridItemProperty {
        match p {
            PersonalProjectProperties::Name => &self.name,
            PersonalProjectProperties::Language => &self.language,
            PersonalProjectProperties::Description => &self.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PersonalProjectName {
    name: String,
    url: String,
}

impl DataGridItemProperty for PersonalProjectName {
    fn cmp_key(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn fmt_html(&self) -> Html {
        html! {
            <a href={self.url.clone()} target="_blank">{&self.name}</a>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GitHubEventWrapper {
    msg: GitHubEventMessage,
    time: String,
    repo: String,
    r#ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitHubEventProperties {
    Message,
    Time,
    Repo,
    Ref,
}

impl From<GitHubEvent> for GitHubEventWrapper {
    fn from(e: GitHubEvent) -> Self {
        let msg = e
            .payload
            .commits
            .last()
            .map(|c| c.message.as_str())
            .unwrap_or("")
            .split(&['\n', '\r'][..])
            .map(|s| html! {{s}})
            .intersperse(html! {<br />})
            .collect::<Vec<Html>>();
        let link = format!(
            "//github.com/{}/commit/{}",
            e.repo.name,
            e.payload
                .commits
                .last()
                .map(|c| c.sha.as_str())
                .unwrap_or("")
        );
        let time = e
            .created_at
            .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
            .naive_local()
            .to_string();
        Self {
            msg: GitHubEventMessage { msg, link },
            time,
            repo: e.repo.name,
            r#ref: e.payload.r#ref,
        }
    }
}

impl DataGridItem for GitHubEventWrapper {
    type Prop = GitHubEventProperties;

    fn prop(&self, p: &Self::Prop) -> &dyn DataGridItemProperty {
        match p {
            GitHubEventProperties::Message => &self.msg,
            GitHubEventProperties::Time => &self.time,
            GitHubEventProperties::Repo => &self.repo,
            GitHubEventProperties::Ref => &self.r#ref,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct GitHubEventMessage {
    msg: Vec<Html>,
    link: String,
}

impl DataGridItemProperty for GitHubEventMessage {
    fn cmp_key(&self) -> Option<&str> {
        None
    }

    fn fmt_html(&self) -> Html {
        html! {
            <a href={self.link.clone()} target="_blank">{self.msg.clone()}</a>
        }
    }
}
