use crate::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="navbar navbar-dark bg-dark">
                <div class="container">
                    <div class="form-row ml-md-auto">
                        <div class="col-auto">
                            <div class="form-row">
                                <div class="col-auto py-2">
                                    <span class="fas fa-envelope"></span>
                                </div>
                                <div class="col-auto py-2">
                                    <a href="mailto:Strawberry_Str@hotmail.com">{"Email"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-auto">
                            <div class="form-row">
                                <div class="col-auto py-2">
                                    <span class="fas fa-rss"></span>
                                </div>
                                <div class="col-auto py-2">
                                    <a href="/blogdata/feed.xml">{"RSS"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-auto">
                            <div class="form-row">
                                <div class="col-auto py-2">
                                    <span class="fas fa-code-branch"></span>
                                </div>
                                <div class="col-auto py-2">
                                    <a href="https://github.com/Berrysoft/">{"Berrysoft"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
