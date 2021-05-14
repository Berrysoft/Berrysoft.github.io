use crate::*;

pub struct Header {
    props: HeaderProperties,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Properties)]
pub struct HeaderProperties {
    pub index: usize,
}

impl Component for Header {
    type Message = ();

    type Properties = HeaderProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let items = HEADER_ITEMS
            .iter()
            .enumerate()
            .map(|(i, (title, link))| {
                let class = if i == self.props.index {
                    "nav-item active"
                } else {
                    "nav-item"
                };
                html! {
                    <li class=class>
                        <a class="nav-link" href=*link>{title}</a>
                    </li>
                }
            })
            .collect::<Vec<Html>>();
        html! {
            <header class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container">
                    <a class="navbar-brand" href="">{"Berrysoft"}</a>
                    <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent">
                        <span class="navbar-toggler-icon"></span>
                    </button>

                    <div id="navbarSupportedContent" class="navbar-collapse collapse" onclick=self.link.callback(|_| collapse_nav())>
                        <ul class="navbar-nav mr-auto">{items}</ul>
                    </div>
                </div>
            </header>
        }
    }
}

static HEADER_ITEMS: [(&str, &str); 3] = [("主页", ""), ("博客", "blog"), ("关于", "about")];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = collapseNav)]
    fn collapse_nav();
}
