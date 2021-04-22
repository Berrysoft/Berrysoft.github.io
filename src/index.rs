use crate::{datagrid::*, *};

pub struct IndexPage {
    link: ComponentLink<Self>,
    github_events: Arc<Vec<github::Event>>,
    github_events_loaded: bool,
    #[allow(dead_code)]
    github_events_task: Option<FetchTask>,
}

pub enum IndexPageMessage {
    GetGitHubEvents,
    GetGitHubEventsSuccess(Vec<github::Event>),
    GetGitHubEventsError,
}

impl IndexPage {
    fn get_github_events(
        callback: Callback<Response<Json<Result<Vec<github::Event>, anyhow::Error>>>>,
    ) -> FetchTask {
        let req = Request::get("//api.github.com/users/berrysoft/events")
            .body(Nothing)
            .unwrap();
        FetchService::fetch(req, callback).unwrap()
    }
}

impl Component for IndexPage {
    type Message = IndexPageMessage;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(IndexPageMessage::GetGitHubEvents);
        Self {
            link,
            github_events: Arc::new(vec![]),
            github_events_loaded: false,
            github_events_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            IndexPageMessage::GetGitHubEvents => {
                self.github_events_loaded = false;
                let handler = self.link.callback(
                    move |res: Response<Json<Result<Vec<github::Event>, anyhow::Error>>>| {
                        let (_, Json(data)) = res.into_parts();
                        match data {
                            Ok(events) => IndexPageMessage::GetGitHubEventsSuccess(events),
                            Err(_) => IndexPageMessage::GetGitHubEventsError,
                        }
                    },
                );
                self.github_events_task = Some(Self::get_github_events(handler));
                true
            }
            IndexPageMessage::GetGitHubEventsSuccess(events) => {
                self.github_events = Arc::new(
                    events
                        .into_iter()
                        .filter(|e| e.r#type == "PushEvent")
                        .take(10)
                        .collect(),
                );
                self.github_events_loaded = true;
                true
            }
            IndexPageMessage::GetGitHubEventsError => {
                self.github_events_loaded = false;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="fade-in fade-in-1">
                    <h1>{"Berrysoft 的 HTML 实验室"}</h1>
                    <p>{"本网站使用 "}<a href="https://www.rust-lang.org/zh-CN/">{"Rust"}</a>{" 开发，有各种各样的实验项目。"}</p>
                </div>
                <div class="fade-in fade-in-2">
                    <h2>{"其它个人开源项目"}</h2>
                    <div class="table-responsive-xl">

                    </div>
                </div>
                <div class="fade-in fade-in-3">
                    <h2>{"GitHub 事件"}</h2>
                    <div class="table-responsive-xl">
                        {
                            if self.github_events_loaded {
                                html! {
                                    <DataGrid<github::Event> data=self.github_events.clone()>
                                        <DataGridColumn<github::Event> header="消息" fmt=box_fmt(|e: &github::Event| e.payload.commits.last().map(|c| c.message.replace("\r\n", "<br/>").replace("\n", "<br/>")).unwrap_or_default())/>
                                        <DataGridColumn<github::Event> header="时间" fmt=box_fmt(|e: &github::Event| e.created_at.with_timezone(&FixedOffset::east(8 * 3600)).naive_local().to_string())/>
                                        <DataGridColumn<github::Event> header="存储库" fmt=box_fmt(|e: &github::Event| e.repo.name.clone())/>
                                    </DataGrid<github::Event>>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
                <div class="fade-in fade-in-4">
                    <h2>{"友情链接"}</h2>
                    <div class="list-group list-group-flush">

                    </div>
                </div>
            </div>
        }
    }
}
