use crate::*;
use std::fmt::Debug;

pub trait DataGridItem {
    fn prop(&self, name: &str) -> &dyn DataGridItemProperty;
}

pub trait DataGridItemProperty {
    fn cmp_key(&self) -> Option<&str>;

    fn fmt_html(&self) -> Html;
}

impl DataGridItemProperty for String {
    fn cmp_key(&self) -> Option<&str> {
        Some(self)
    }

    fn fmt_html(&self) -> Html {
        html! {{self}}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortOrdering {
    None,
    Ascending,
    Descending,
}

#[derive(Debug)]
pub struct DataGrid<T: DataGridItem + Clone + 'static> {
    props: DataGridProperties<T>,
    link: ComponentLink<Self>,
    sort_prop: Option<String>,
    sort_order: SortOrdering,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataGridMessage {
    ColumnClick(bool, String),
}

#[derive(Debug, Clone, Properties)]
pub struct DataGridProperties<T: DataGridItem + Clone + 'static> {
    pub children: ChildrenWithProps<DataGridColumn>,
    pub data: Rc<Vec<T>>,
}

impl<T: DataGridItem + Clone + 'static> Component for DataGrid<T> {
    type Message = DataGridMessage;

    type Properties = DataGridProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            sort_prop: None,
            sort_order: SortOrdering::None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DataGridMessage::ColumnClick(allow, prop) => {
                if allow {
                    if let Some(old_prop) = &self.sort_prop {
                        if old_prop == &prop {
                            self.sort_order = match self.sort_order {
                                SortOrdering::None => SortOrdering::Ascending,
                                SortOrdering::Ascending => SortOrdering::Descending,
                                SortOrdering::Descending => SortOrdering::None,
                            }
                        } else {
                            self.sort_prop = Some(prop);
                            self.sort_order = SortOrdering::Ascending;
                        }
                    } else {
                        self.sort_prop = Some(prop);
                        self.sort_order = SortOrdering::Ascending;
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        log::debug!("DataGrid view: {}", self.props.data.len());
        let icon_class = match self.sort_order {
            SortOrdering::None => "fas",
            SortOrdering::Ascending => "fas fa-chevron-up",
            SortOrdering::Descending => "fas fa-chevron-down",
        };
        let cols = self
            .props
            .children
            .iter()
            .map(|c| {
                let style = if c.props.sortable {
                    "cursor: pointer"
                } else {
                    "cursor: auto"
                };
                let icon_hidden = !c.props.sortable
                    || self.sort_order == SortOrdering::None
                    || self.sort_prop.as_ref() != Some(&c.props.prop);
                let c_clone = c.clone();
                let callback = self.link.callback(move |_| {
                    DataGridMessage::ColumnClick(c_clone.props.sortable, c_clone.props.prop.clone())
                });
                html! {
                    <th scope="col" style=style onclick=callback>
                        <div class="row">
                            <div class="col">
                                {c.clone()}
                            </div>
                            <div class="col-auto" hidden=icon_hidden>
                                <span class=icon_class></span>
                            </div>
                        </div>
                    </th>
                }
            })
            .collect::<Vec<Html>>();
        let mut row_data = (*self.props.data).clone();
        if let Some(prop) = &self.sort_prop {
            match self.sort_order {
                SortOrdering::None => {}
                SortOrdering::Ascending => row_data.sort_unstable_by(|lhs, rhs| {
                    let lprop = lhs.prop(prop).cmp_key().unwrap();
                    let rprop = rhs.prop(prop).cmp_key().unwrap();
                    lprop.cmp(rprop)
                }),
                SortOrdering::Descending => row_data.sort_unstable_by(|lhs, rhs| {
                    let lprop = lhs.prop(prop).cmp_key().unwrap();
                    let rprop = rhs.prop(prop).cmp_key().unwrap();
                    rprop.cmp(lprop)
                }),
            }
        }
        let rows = row_data
            .into_iter()
            .map(|d| {
                let cols = self
                    .props
                    .children
                    .iter()
                    .map(|c| {
                        let prop = d.prop(&c.props.prop);
                        html! {<td>{prop.fmt_html()}</td>}
                    })
                    .collect::<Vec<Html>>();
                html! {
                    <tr>{cols}</tr>
                }
            })
            .collect::<Vec<Html>>();
        html! {
            <table class="table table-hover">
                <thead>
                    {cols}
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        }
    }
}

#[derive(Debug)]
pub struct DataGridColumn {
    props: DataGridColumnProperties,
}

#[derive(Debug, Clone, Properties)]
pub struct DataGridColumnProperties {
    pub header: String,
    pub prop: String,
    #[prop_or_default]
    pub sortable: bool,
}

impl Component for DataGridColumn {
    type Message = ();

    type Properties = DataGridColumnProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {&self.props.header}
    }
}
