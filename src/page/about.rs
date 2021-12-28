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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            libs: JsonFetcher::new("/data/libraries.json", ctx, |msg| {
                AboutPageMessage::GetLibraries(msg)
            }),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AboutPageMessage::GetLibraries(msg) => {
                self.libs.update(msg);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let libraries = self
            .libs
            .get()
            .map(|libs| {
                html! {
                    <DataGrid<LibraryWrapper> data={libs}>
                        <DataGridColumn<LibraryWrapper> header="名称" prop={LibraryPerperties::Name} sortable=true/>
                        <DataGridColumn<LibraryWrapper> header="许可证" prop={LibraryPerperties::License} sortable=true/>
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

#[derive(Debug, Clone, PartialEq)]
pub struct LibraryWrapper {
    name: LibraryName,
    license: LibraryLicense,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryPerperties {
    Name,
    License,
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
    type Prop = LibraryPerperties;

    fn prop(&self, p: &Self::Prop) -> &dyn DataGridItemProperty {
        match p {
            LibraryPerperties::Name => &self.name,
            LibraryPerperties::License => &self.license,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct LibraryName {
    name: String,
    url: String,
}

impl DataGridItemProperty for LibraryName {
    fn cmp_key(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn fmt_html(&self) -> Html {
        html! {
            <a href={self.url.clone()} target="_blank">{&self.name}</a>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct LibraryLicense {
    license: String,
    license_url: Option<String>,
}

impl DataGridItemProperty for LibraryLicense {
    fn cmp_key(&self) -> Option<&str> {
        Some(&self.license)
    }

    fn fmt_html(&self) -> Html {
        if let Some(url) = &self.license_url {
            html! {<a href={url.clone()} target="_blank">{&self.license}</a>}
        } else {
            html! {{&self.license}}
        }
    }
}
