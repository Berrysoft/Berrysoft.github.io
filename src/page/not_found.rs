use crate::{layout::*, *};

pub struct NotFoundPage;

#[derive(Clone, PartialEq, Properties)]
pub struct NotFoundProperties {
    pub route: Option<url::Url>,
}

impl Component for NotFoundPage {
    type Message = ();

    type Properties = NotFoundProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let issue = ctx
            .props()
            .route
            .as_ref()
            .map(|route| html! {
                <p>
                    {"想要让我增加 "}
                    <code>{route.path()}</code>
                    {" 页面？快去提 "}
                    <a href="https://github.com/Berrysoft/Berrysoft.github.io/issues" target="_blank">{"issue"}</a>
                    {" 吧！"}
                </p>
            })
            .unwrap_or_default();
        html! {
            <>
                <Header index={std::usize::MAX}/>
                <div class="container">
                    <h1>{"404"}</h1>
                    <p>{"对不起，这里什么也没有。"}</p>
                    {issue}
                </div>
                <Footer />
            </>
        }
    }
}
