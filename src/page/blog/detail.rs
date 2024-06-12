use crate::{data::*, *};
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use url::Url;

pub struct BlogDetailPage {
    blogs: TextFetcher,
    text: TextFetcher,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct BlogDetailProperties {
    pub name: String,
}

pub enum BlogDetailMessage {
    Blogs(TextFetcherMessage),
    Text(TextFetcherMessage),
}

impl Component for BlogDetailPage {
    type Message = BlogDetailMessage;

    type Properties = BlogDetailProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let uri = format!("/blogdata/{}.md", ctx.props().name);
        Self {
            blogs: TextFetcher::new("/blogdata/feed.xml", ctx, BlogDetailMessage::Blogs),
            text: TextFetcher::new(&uri, ctx, BlogDetailMessage::Text),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BlogDetailMessage::Blogs(msg) => {
                self.blogs.update(msg);
                true
            }
            BlogDetailMessage::Text(msg) => {
                self.text.update(msg);
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        log::debug!("Blog detail rendered");
        // Make analyzer happy
        #[allow(unused_unsafe)]
        unsafe {
            highlight_all();
            math_all();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = self
            .blogs
            .get()
            .and_then(|blogs| {
                BlogItem::parse_rss(blogs)
                    .into_iter()
                    .find(|item| item.filename == ctx.props().name)
            })
            .map(|item| {
                let time_str = item.time.naive_local().to_string();
                html! {
                    <>
                        <h1>{item.title}</h1>
                        <p class="text-secondary">
                            <time datetime={time_str.clone()}>{&time_str}</time>
                        </p>
                    </>
                }
            })
            .unwrap_or_default();
        let text = self
            .text
            .get()
            .map(|text| {
                let parser = Parser::new_ext(text, Options::ENABLE_TABLES);
                let parser = parser.map(|event| match event {
                    Event::Start(tag) => {
                        log::debug!("{:?}", tag);
                        let tag = match tag {
                            Tag::Image {
                                link_type,
                                dest_url,
                                title,
                                id,
                            } => {
                                let dest_url = match Url::parse(&dest_url) {
                                    Ok(_) => dest_url,
                                    Err(e) => match e {
                                        url::ParseError::RelativeUrlWithoutBase => Url::parse(
                                            &gloo_utils::window().location().origin().unwrap(),
                                        )
                                        .unwrap()
                                        .join("/blogdata/")
                                        .unwrap()
                                        .join(&dest_url)
                                        .unwrap()
                                        .to_string()
                                        .into(),
                                        _ => dest_url,
                                    },
                                };
                                Tag::Image {
                                    link_type,
                                    dest_url,
                                    title,
                                    id,
                                }
                            }
                            _ => tag,
                        };
                        Event::Start(tag)
                    }
                    _ => event,
                });
                let mut out = String::new();
                html::push_html(&mut out, parser);
                parse_html(&out)
            })
            .unwrap_or_default();
        html! {
            <div class="container">
                <article>
                    <div class="fade-in fade-in-1">{title}</div>
                    <div class="fade-in fade-in-2">{text}</div>
                </article>
            </div>
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = MathJax, js_name = typeset)]
    fn math_all();
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll)]
    fn highlight_all();
}
