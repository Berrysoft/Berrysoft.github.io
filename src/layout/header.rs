use crate::*;

pub struct Header;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct HeaderProperties {
    pub index: usize,
}

impl Component for Header {
    type Message = ();

    type Properties = HeaderProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items = HEADER_ITEMS
            .iter()
            .enumerate()
            .map(|(i, (title, link))| {
                let class = if i == ctx.props().index {
                    "nav-item active"
                } else {
                    "nav-item"
                };
                html! {
                    <li class={class}>
                        <a class="nav-link" href={*link}>{title}</a>
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

                    <div id="navbarSupportedContent" class="navbar-collapse collapse" onclick={ctx.link().callback(|_| collapse_nav())}>
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
