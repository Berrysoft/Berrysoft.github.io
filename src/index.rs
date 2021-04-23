use crate::{data::*, datagrid::*, fetch::*, footer::*, header::*, *};

pub struct IndexPage {
    projects: JsonFetcher<PersonalProject>,
    github_events: JsonFetcher<github::Event>,
    links: JsonFetcher<FriendLink>,
}

pub enum IndexPageMessage {
    GetProjects(JsonFetcherMessage<PersonalProject>),
    GetGitHubEvents(JsonFetcherMessage<github::Event>),
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
        let projects = if let Some(projects) = self.projects.get() {
            html! {
                <DataGrid<PersonalProject> data=projects>
                    <DataGridColumn<PersonalProject> header="名称" fmt=box_fmt(|p: &PersonalProject| format!("<a href=\"{}\" target=\"_blank\">{}</a>", p.url, p.name))/>
                    <DataGridColumn<PersonalProject> header="主要语言" fmt=box_fmt(|p: &PersonalProject| p.language.clone())/>
                    <DataGridColumn<PersonalProject> header="简介" fmt=box_fmt(|p: &PersonalProject| p.description.clone())/>
                </DataGrid<PersonalProject>>
            }
        } else {
            html! {}
        };
        let github_events_node = if let Some(events) = self.github_events.get() {
            html! {
                <DataGrid<github::Event> data=events>
                    <DataGridColumn<github::Event> header="消息" fmt=box_fmt(|e: &github::Event| {
                        let msg = e.payload.commits.last().map(|c| c.message.replace("\r\n", "<br/>").replace("\n", "<br/>")).unwrap_or_default();
                        let link = format!("//github.com/{}/commit/{}", e.repo.name, e.payload.commits.last().map(|c| c.sha.clone()).unwrap_or_default());
                        format!("<a href=\"{}\" target=\"_blank\">{}</a>", link, msg)
                    })/>
                    <DataGridColumn<github::Event> header="时间" fmt=box_fmt(|e: &github::Event| e.created_at.with_timezone(&FixedOffset::east(8 * 3600)).naive_local().to_string())/>
                    <DataGridColumn<github::Event> header="存储库" fmt=box_fmt(|e: &github::Event| e.repo.name.clone())/>
                </DataGrid<github::Event>>
            }
        } else {
            html! {}
        };
        let friend_links = if let Some(links) = self.links.get() {
            links.iter().map(|link| html! {
                <a class="list-group-item list-group-item-action" href=link.url.clone() target="_blank">{&format!("{} - {}", link.name, link.title)}</a>
            }).collect()
        } else {
            vec![]
        };
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
