use crate::tred::*;
use crate::*;

pub struct IndexPage {}

impl Component for IndexPage {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="fade-in fade-in-1">
                    <h1><Tred k="indexTitle"/></h1>

                    <p><Tred k="indexDescription"/></p>
                </div>
            </div>
        }
    }
}
