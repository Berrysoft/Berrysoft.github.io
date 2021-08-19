use crate::{data::*, layout::*, *};
use pulldown_cmark::{html, Parser};

pub struct BlogDetailPage {
    props: BlogDetailProperties,
    blogs: TextFetcher,
    text: TextFetcher,
}

#[derive(Debug, Clone, Properties)]
pub struct BlogDetailProperties {
    pub name: String,
}

pub enum BlogDetailMessage {
    GetBlogs(TextFetcherMessage),
    GetText(TextFetcherMessage),
}

impl Component for BlogDetailPage {
    type Message = BlogDetailMessage;

    type Properties = BlogDetailProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let uri = format!("/blogdata/{}.md", props.name);
        Self {
            props,
            blogs: TextFetcher::new("/blogdata/feed.xml", link.clone(), |msg| {
                BlogDetailMessage::GetBlogs(msg)
            }),
            text: TextFetcher::new(&uri, link, BlogDetailMessage::GetText),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BlogDetailMessage::GetBlogs(msg) => {
                self.blogs.update(msg);
                true
            }
            BlogDetailMessage::GetText(msg) => {
                self.text.update(msg);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        log::debug!("Blog defail rendered");
        // Make analyzer happy
        #[allow(unused_unsafe)]
        unsafe {
            highlight_all();
            math_all();
        }
    }

    fn view(&self) -> Html {
        let title = self
            .blogs
            .get()
            .and_then(|blogs| {
                BlogItem::parse_rss(blogs)
                    .into_iter()
                    .find(|item| item.filename == self.props.name)
            })
            .map(|item| {
                let time_str = item.time.naive_local().to_string();
                html! {
                    <>
                        <h1>{item.title}</h1>
                        <p class="text-secondary">
                            <time datetime=time_str.clone()>{&time_str}</time>
                        </p>
                    </>
                }
            })
            .unwrap_or_default();
        let text = self
            .text
            .get()
            .map(|text| {
                let parser = Parser::new(text);
                let mut out = String::new();
                html::push_html(&mut out, parser);
                parse_html(&out)
            })
            .unwrap_or_default();
        html! {
            <>
                <Header index=1/>
                <div class="container">
                    <article>
                        <div class="fade-in fade-in-1">{title}</div>
                        <div class="fade-in fade-in-2">{text}</div>
                    </article>
                </div>
                <Footer />
            </>
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
