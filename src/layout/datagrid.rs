use crate::*;
use std::{fmt::Debug, marker::PhantomData};

pub trait DataGridItem {
    type Prop: Debug + Clone + Eq;

    fn prop(&self, p: &Self::Prop) -> &dyn DataGridItemProperty;
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
pub struct DataGrid<T: DataGridItem + PartialEq + Clone + 'static> {
    sort_prop: Option<T::Prop>,
    sort_order: SortOrdering,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataGridMessage<T: DataGridItem + PartialEq + Clone + 'static> {
    ColumnClick(bool, T::Prop),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DataGridProperties<T: DataGridItem + PartialEq + Clone + 'static> {
    pub children: ChildrenWithProps<DataGridColumn<T>>,
    pub data: Rc<Vec<T>>,
}

impl<T: DataGridItem + PartialEq + Clone + 'static> Component for DataGrid<T> {
    type Message = DataGridMessage<T>;

    type Properties = DataGridProperties<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            sort_prop: None,
            sort_order: SortOrdering::None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("DataGrid view: {}", ctx.props().data.len());
        let icon_class = match self.sort_order {
            SortOrdering::None => "fas",
            SortOrdering::Ascending => "fas fa-chevron-up",
            SortOrdering::Descending => "fas fa-chevron-down",
        };
        let cols = ctx
            .props()
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
                let callback = ctx.link().callback(move |_| {
                    DataGridMessage::ColumnClick(c_clone.props.sortable, c_clone.props.prop.clone())
                });
                html! {
                    <th scope="col" style={style} onclick={callback}>
                        <div class="row">
                            <div class="col">
                                {c.clone()}
                            </div>
                            <div class="col-auto" hidden={icon_hidden}>
                                <span class={icon_class}></span>
                            </div>
                        </div>
                    </th>
                }
            })
            .collect::<Vec<Html>>();
        let mut row_data = (*ctx.props().data).clone();
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
                let cols = ctx
                    .props()
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
pub struct DataGridColumn<T: DataGridItem + PartialEq + Clone + 'static> {
    _p: PhantomData<T>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DataGridColumnProperties<T: DataGridItem + PartialEq + Clone + 'static> {
    pub header: String,
    pub prop: T::Prop,
    #[prop_or_default]
    pub sortable: bool,
}

impl<T: DataGridItem + PartialEq + Clone + 'static> Component for DataGridColumn<T> {
    type Message = ();

    type Properties = DataGridColumnProperties<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { _p: PhantomData }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {&ctx.props().header}
    }
}
