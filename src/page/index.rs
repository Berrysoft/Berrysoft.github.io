use crate::{data::*, layout::*, *};

pub struct IndexPage {
    projects: JsonFetcher<PersonalProject, PersonalProjectWrapper>,
    github_events: JsonFetcher<GitHubEvent, GitHubEventWrapper>,
    links: JsonFetcher<FriendLink, FriendLink>,
}

pub enum IndexPageMessage {
    GetProjects(JsonFetcherMessage<PersonalProject, PersonalProjectWrapper>),
    GetGitHubEvents(JsonFetcherMessage<GitHubEvent, GitHubEventWrapper>),
    GetFriendLinks(JsonFetcherMessage<FriendLink, FriendLink>),
}

impl Component for IndexPage {
    type Message = IndexPageMessage;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            projects: JsonFetcher::new(
                "/data/projects.json",
                link.clone(),
                IndexPageMessage::GetProjects,
            ),
            github_events: JsonFetcher::new(
                "//api.github.com/users/berrysoft/events",
                link.clone(),
                IndexPageMessage::GetGitHubEvents,
            ),
            links: JsonFetcher::new("/data/links.json", link, IndexPageMessage::GetFriendLinks),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            IndexPageMessage::GetProjects(msg) => {
                self.projects.update(msg);
                true
            }
            IndexPageMessage::GetGitHubEvents(mut msg) => {
                if let Ok(events) = msg {
                    msg = Ok(events
                        .into_iter()
                        .filter(|e| e.r#type == "PushEvent")
                        .collect());
                }
                self.github_events.update(msg);
                true
            }
            IndexPageMessage::GetFriendLinks(msg) => {
                self.links.update(msg);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let projects = self
            .projects
            .get()
            .map(|projects| {
                html! {
                    <DataGrid<PersonalProjectWrapper> data=projects>
                        <DataGridColumn header="名称" prop="name"/>
                        <DataGridColumn header="主要语言" prop="language"/>
                        <DataGridColumn header="简介" prop="description"/>
                    </DataGrid<PersonalProjectWrapper>>
                }
            })
            .unwrap_or_default();
        let github_events_node = self
            .github_events
            .get()
            .map(|events| {
                html! {
                    <DataGrid<GitHubEventWrapper> data=events>
                        <DataGridColumn header="消息" prop="msg"/>
                        <DataGridColumn header="时间" prop="time"/>
                        <DataGridColumn header="存储库" prop="repo"/>
                    </DataGrid<GitHubEventWrapper>>
                }
            })
            .unwrap_or_default();
        let friend_links = self
            .links
            .get()
            .map(|links| {
                links.iter().map(|link| html! {
                    <a class="list-group-item list-group-item-action" href=link.url.clone() target="_blank">
                        {&format!("{} - {}", link.name, link.title)}
                    </a>
                }).collect::<Vec<Html>>()
            })
            .unwrap_or_default();
        html! {
            <>
                <Header index=0/>
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
                <Footer/>
            </>
        }
    }
}

#[derive(Debug, Clone)]
pub struct PersonalProjectWrapper {
    name: PersonalProjectName,
    language: String,
    description: String,
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
    fn prop(&self, name: &str) -> Box<dyn DataGridItemProperty> {
        match name {
            "name" => Box::new(self.name.clone()),
            "language" => Box::new(self.language.clone()),
            "description" => Box::new(self.description.clone()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct PersonalProjectName {
    name: String,
    url: String,
}

impl DataGridItemProperty for PersonalProjectName {
    fn fmt_html(&self) -> Html {
        html! {
            <a href=self.url.clone() target="_blank">{&self.name}</a>
        }
    }
}

#[derive(Debug, Clone)]
pub struct GitHubEventWrapper {
    msg: GitHubEventMessage,
    time: String,
    repo: String,
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
            .with_timezone(&FixedOffset::east(8 * 3600))
            .naive_local()
            .to_string();
        Self {
            msg: GitHubEventMessage { msg, link },
            time,
            repo: e.repo.name,
        }
    }
}

impl DataGridItem for GitHubEventWrapper {
    fn prop(&self, name: &str) -> Box<dyn DataGridItemProperty> {
        match name {
            "msg" => Box::new(self.msg.clone()),
            "time" => Box::new(self.time.clone()),
            "repo" => Box::new(self.repo.clone()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct GitHubEventMessage {
    msg: Vec<Html>,
    link: String,
}

impl DataGridItemProperty for GitHubEventMessage {
    fn fmt_html(&self) -> Html {
        html! {
            <a href=self.link.clone() target="_blank">{self.msg.clone()}</a>
        }
    }
}
