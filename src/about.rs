use crate::{data::*, datagrid::*, fetch::*, footer::*, header::*, *};

pub struct AboutPage {
    libs: JsonFetcher<Library>,
}

pub enum AboutPageMessage {
    GetLibraries(JsonFetcherMessage<Library>),
}

impl Component for AboutPage {
    type Message = AboutPageMessage;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            libs: JsonFetcher::new("/data/libraries.json", link, |msg| {
                AboutPageMessage::GetLibraries(msg)
            }),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AboutPageMessage::GetLibraries(msg) => {
                self.libs.update(msg);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let libraries = if let Some(libs) = self.libs.get() {
            html! {
                <DataGrid<Library> data=libs>
                    <DataGridColumn<Library> header="名称" fmt=box_fmt(|lib: &Library| format!("<a href=\"{}\">{}</a>", lib.url, lib.name))/>
                    <DataGridColumn<Library> header="许可证" fmt=box_fmt(|lib: &Library| {
                        if let Some(url) = &lib.license_url {
                            format!("<a href=\"{}\">{}</a>", url, lib.license)
                        } else {
                            lib.license.clone()
                        }
                    })/>
                </DataGrid<Library>>
            }
        } else {
            html! {}
        };
        html! {
            <>
                <Header index=2/>
                <div class="container">
                    <div class="fade-in fade-in-1">
                        <h1>{"关于本网站"}</h1>
                        <p class="text-secondary">
                            {"Copyright (c) 2019-2020 Berrysoft"}
                            <br />
                            <a href="//github.com/Berrysoft/Berrysoft.github.io">{"项目源代码"}</a>
                        </p>
                    </div>
                    <div class="fade-in fade-in-2">
                        <h2>{"包含的开源库"}</h2>
                        <div class="table-responsive-xl">{libraries}</div>
                    </div>
                </div>
                <Footer/>
            </>
        }
    }
}
