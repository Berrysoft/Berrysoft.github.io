use crate::{layout::*, *};

pub struct NotFoundPage {
    props: NotFoundProperties,
}

#[derive(Debug, Clone, Properties)]
pub struct NotFoundProperties {
    pub route: Option<String>,
}

impl Component for NotFoundPage {
    type Message = ();

    type Properties = NotFoundProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let issue = self
            .props
            .route
            .as_ref()
            .map(|route| html! {
                <p>
                    {"想要让我增加 "}
                    <code>{route.as_str()}</code>
                    {" 页面？快去提 "}
                    <a href="https://github.com/Berrysoft/Berrysoft.github.io/issues" target="_blank">{"issue"}</a>
                    {" 吧！"}
                </p>
            })
            .unwrap_or_default();
        html! {
            <>
                <Header index=std::usize::MAX/>
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
