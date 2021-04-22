use crate::{fetch::*, footer::*, header::*, *};

pub mod detail;

pub struct BlogPage {
    blogs: TextFetcher,
}

pub enum BlogPageMessage {
    GetBlogs(TextFetcherMessage),
}

impl Component for BlogPage {
    type Message = BlogPageMessage;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            blogs: TextFetcher::new("/blogdata/feed.xml", link, |msg| {
                BlogPageMessage::GetBlogs(msg)
            }),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BlogPageMessage::GetBlogs(msg) => {
                self.blogs.update(msg);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let blogs = if let Some(blogs) = self.blogs.get() {
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
                    html!{
                        <a class="list-group-item list-group-item-action" href=format!("/blog/{}", filename)>
                            <h2>{item.title.unwrap_or_default()}</h2>
                            <p class="text-secondary">
                                <time datetime=time.naive_local().to_string()>{time.naive_local().to_string()}</time>
                            </p>
                            <p>{item.description.unwrap_or_default()}</p>
                        </a>
                    }
                })
                .collect::<Vec<Html>>()
        } else {
            vec![]
        };
        html! {
            <>
                <Header index=1/>
                <div class="container">
                    <div class="fade-in fade-in-1">
                        <h1>{"博客"}</h1>
                    </div>
                    <div class="fade-in fade-in-2">
                        <div class="list-group list-group-flush">{blogs}</div>
                    </div>
                </div>
                <Footer />
            </>
        }
    }
}
