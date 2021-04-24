use crate::{data::*, layout::*, *};

pub struct IndexPage {
    projects: JsonFetcher<PersonalProject>,
    github_events: JsonFetcher<GitHubEvent>,
    links: JsonFetcher<FriendLink>,
}

pub enum IndexPageMessage {
    GetProjects(JsonFetcherMessage<PersonalProject>),
    GetGitHubEvents(JsonFetcherMessage<GitHubEvent>),
    GetFriendLinks(JsonFetcherMessage<FriendLink>),
}

impl Component for IndexPage {
    type Message = IndexPageMessage;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            projects: JsonFetcher::new("/data/projects.json", link.clone(), |msg| {
                IndexPageMessage::GetProjects(msg)
            }),
            github_events: JsonFetcher::new(
                "//api.github.com/users/berrysoft/events",
                link.clone(),
                |msg| IndexPageMessage::GetGitHubEvents(msg),
            ),
            links: JsonFetcher::new("/data/links.json", link.clone(), |msg| {
                IndexPageMessage::GetFriendLinks(msg)
            }),
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
                    <DataGrid<PersonalProject> data=projects>
                        <DataGridColumn<PersonalProject> header="名称" fmt=box_fmt(|p: &PersonalProject| format!("<a href=\"{}\" target=\"_blank\">{}</a>", p.url, p.name))/>
                        <DataGridColumn<PersonalProject> header="主要语言" fmt=box_fmt(|p: &PersonalProject| p.language.clone())/>
                        <DataGridColumn<PersonalProject> header="简介" fmt=box_fmt(|p: &PersonalProject| p.description.clone())/>
                    </DataGrid<PersonalProject>>
                }
            })
            .unwrap_or_default();
        let github_events_node = self
            .github_events
            .get()
            .map(|events| {
                html! {
                    <DataGrid<GitHubEvent> data=events>
                        <DataGridColumn<GitHubEvent> header="消息" fmt=box_fmt(|e: &GitHubEvent| {
                            let msg = e.payload.commits.last().map(|c| c.message.replace("\r\n", "<br/>").replace("\n", "<br/>")).unwrap_or_default();
                            let link = format!("//github.com/{}/commit/{}", e.repo.name, e.payload.commits.last().map(|c| c.sha.clone()).unwrap_or_default());
                            format!("<a href=\"{}\" target=\"_blank\">{}</a>", link, msg)
                        })/>
                        <DataGridColumn<GitHubEvent> header="时间" fmt=box_fmt(|e: &GitHubEvent| e.created_at.with_timezone(&FixedOffset::east(8 * 3600)).naive_local().to_string())/>
                        <DataGridColumn<GitHubEvent> header="存储库" fmt=box_fmt(|e: &GitHubEvent| e.repo.name.clone())/>
                    </DataGrid<GitHubEvent>>
                }
            })
            .unwrap_or_default();
        let friend_links = self
            .links
            .get()
            .map(|links| {
                links.iter().map(|link| html! {
                    <a class="list-group-item list-group-item-action" href=link.url.clone() target="_blank">{&format!("{} - {}", link.name, link.title)}</a>
                }).collect::<Vec<Html>>()
            })
            .unwrap_or_default();
        html! {
            <>
                <Header index=0/>
                <div class="container">
                    <div class="fade-in fade-in-1">
                        <h1>{"Berrysoft 的 HTML 实验室"}</h1>
                        <p>{"本网站使用 "}<a href="https://www.rust-lang.org/zh-CN/" target="_blank">{"Rust"}</a>{" 开发，有各种各样的实验项目。"}</p>
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
