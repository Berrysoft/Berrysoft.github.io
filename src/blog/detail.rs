use crate::{blog::BlogItem, fetch::*, footer::*, header::*, *};
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
            text: TextFetcher::new(uri, link.clone(), |msg| BlogDetailMessage::GetText(msg)),
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
        // Make analyzer happy
        #[allow(unused_unsafe)]
        unsafe {
            highlight_all();
            math_all();
        }
    }

    fn view(&self) -> Html {
        let title = if let Some(blogs) = self.blogs.get() {
            let item = BlogItem::parse_rss(blogs)
                .into_iter()
                .filter(|item| item.filename == self.props.name)
                .next()
                .unwrap();
            html! {
                <>
                    <h1>{item.title}</h1>
                    <p class="text-secondary">
                        <time datetime=item.time.naive_local().to_string()>{item.time.naive_local().to_string()}</time>
                    </p>
                </>
            }
        } else {
            html! {}
        };
        let text = if let Some(text) = self.text.get() {
            let parser = Parser::new(&text);
            let mut out = String::new();
            html::push_html(&mut out, parser);
            let dom = web_sys::DomParser::new()
                .unwrap()
                .parse_from_string(
                    &format!("<parse>{}</parse>", out),
                    web_sys::SupportedType::TextHtml,
                )
                .unwrap();
            let body = dom.body().unwrap();
            yew::virtual_dom::VNode::VRef(body.children().get_with_index(0).unwrap().into())
        } else {
            html! {}
        };
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
