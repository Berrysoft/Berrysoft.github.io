use crate::{footer::*, header::*, *};

pub struct AboutPage;

impl Component for AboutPage {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header/>
                <div class="container">
                    <div class="fade-in fade-in-1">
                        <h1>{"关于本网站"}</h1>
                        <p class="text-secondary">
                            {"Copyright (c) 2019-2020 Berrysoft"}
                            <br />
                            <a href="//github.com/Berrysoft/Berrysoft.github.io">{"项目源代码"}</a>
                        </p>
                    </div>
                </div>
                <Footer/>
            </>
        }
    }
}
