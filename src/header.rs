use crate::*;

pub struct Header {
    link: ComponentLink<Self>,
}

impl Component for Header {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <header class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container">
                    <a class="navbar-brand" href="">{"Berrysoft"}</a>
                    <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent">
                        <span class="navbar-toggler-icon"></span>
                    </button>

                    <div id="navbarSupportedContent" class="navbar-collapse collapse" onclick=self.link.callback(|_| collapse_nav())>
                        <ul class="navbar-nav mr-auto">
                            <li class="nav-item">
                                <a class="nav-link" href="/">{"主页"}</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </header>
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = collapseNav)]
    fn collapse_nav();
}
