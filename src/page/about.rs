use crate::{data::*, layout::*, *};

pub struct AboutPage {
    libs: JsonFetcher<Library, LibraryWrapper>,
}

pub enum AboutPageMessage {
    GetLibraries(JsonFetcherMessage<Library, LibraryWrapper>),
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
        let libraries = self
            .libs
            .get()
            .map(|libs| {
                html! {
                    <DataGrid<LibraryWrapper> data=libs>
                        <DataGridColumn header="名称" prop="name" sortable=true/>
                        <DataGridColumn header="许可证" prop="license" sortable=true/>
                    </DataGrid<LibraryWrapper>>
                }
            })
            .unwrap_or_default();
        html! {
            <>
                <Header index=2/>
                <div class="container">
                    <div class="fade-in fade-in-1">
                        <h1>{"关于本网站"}</h1>
                        <p class="text-secondary">
                            {"Copyright (c) 2019-2021 Berrysoft"}
                            <br />
                            <a href="//github.com/Berrysoft/Berrysoft.github.io" target="_blank">{"项目源代码"}</a>
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

#[derive(Debug, Clone)]
pub struct LibraryWrapper {
    name: LibraryName,
    license: LibraryLicense,
}

impl From<Library> for LibraryWrapper {
    fn from(lib: Library) -> Self {
        Self {
            name: LibraryName {
                name: lib.name,
                url: lib.url,
            },
            license: LibraryLicense {
                license: lib.license,
                license_url: lib.license_url,
            },
        }
    }
}

impl DataGridItem for LibraryWrapper {
    fn prop(&self, name: &str) -> &dyn DataGridItemProperty {
        match name {
            "name" => &self.name,
            "license" => &self.license,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct LibraryName {
    name: String,
    url: String,
}

impl DataGridItemProperty for LibraryName {
    fn cmp_key(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn fmt_html(&self) -> Html {
        html! {
            <a href=self.url.clone() target="_blank">{&self.name}</a>
        }
    }
}

#[derive(Debug, Clone)]
struct LibraryLicense {
    license: String,
    license_url: Option<String>,
}

impl DataGridItemProperty for LibraryLicense {
    fn cmp_key(&self) -> Option<String> {
        Some(self.license.clone())
    }

    fn fmt_html(&self) -> Html {
        html! {
            if let Some(url) = &self.license_url {
                html! {<a href=url.clone() target="_blank">{&self.license}</a>}
            } else {
                html! {{&self.license}}
            }
        }
    }
}
