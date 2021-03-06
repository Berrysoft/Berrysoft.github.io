use crate::{data::*, layout::*, *};

mod detail;
pub use detail::*;

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
        let blogs = self
            .blogs
            .get()
            .map(|blogs| {
                BlogItem::parse_rss(blogs)
                    .into_iter()
                    .map(|item| {
                        let time_str = item.time.naive_local().to_string();
                        html!{
                            <a class="list-group-item list-group-item-action" href=format!("/blog/{}", item.filename)>
                                <h2>{item.title}</h2>
                                <p class="text-secondary">
                                    <time datetime=time_str.clone()>{&time_str}</time>
                                </p>
                                <p>{item.description}</p>
                            </a>
                        }
                    })
                    .collect::<Vec<Html>>()
            })
            .unwrap_or_default();
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
