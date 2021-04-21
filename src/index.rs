use crate::*;

pub struct IndexPage {}

impl Component for IndexPage {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="fade-in fade-in-1">
                    <h1>{"Title"}</h1>

                    <p>{"Description"}</p>
                </div>
            </div>
        }
    }
}
