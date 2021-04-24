use crate::{data::*, *};
use std::fmt::Debug;

#[derive(Debug)]
pub struct DataGrid<T: Clone + 'static> {
    props: DataGridProperties<T>,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Properties)]
pub struct DataGridProperties<T: Clone + 'static> {
    pub children: ChildrenWithProps<DataGridColumn<T>>,
    pub data: Arc<Vec<T>>,
}

impl<T: Clone + 'static> Component for DataGrid<T> {
    type Message = ();

    type Properties = DataGridProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        console_log!("DataGrid view: {}", self.props.data.len());
        let rows = self
            .props
            .data
            .iter()
            .map(|d| {
                let cols = self
                    .props
                    .children
                    .iter()
                    .map(|c| {
                        let child = parse_html(&c.props.fmt.fmt(d));
                        html! {<td>{child}</td>}
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
                    {self.props.children.clone()}
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        }
    }
}

#[derive(Debug)]
pub struct DataGridColumn<T: Clone + 'static> {
    props: DataGridColumnProperties<T>,
}

#[derive(Clone, Properties)]
pub struct DataGridColumnProperties<T: Clone + 'static> {
    pub header: String,
    pub fmt: Arc<dyn DataGridColumnFormatter<T>>,
}

pub trait DataGridColumnFormatter<T: Clone + 'static> {
    fn fmt(&self, item: &T) -> String;
}

impl<T: Clone + 'static> Debug for DataGridColumnProperties<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("DataGridColumnProperties")
    }
}

impl<T: Clone + 'static> Component for DataGridColumn<T> {
    type Message = ();

    type Properties = DataGridColumnProperties<T>;

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
        html! {
            <th scope="col">{&self.props.header}</th>
        }
    }
}

struct FuncFormatter<T: Clone + 'static> {
    func: Box<dyn Fn(&T) -> String>,
}

impl<T: Clone + 'static> DataGridColumnFormatter<T> for FuncFormatter<T> {
    fn fmt(&self, item: &T) -> String {
        (self.func)(item)
    }
}

pub fn box_fmt<T: Clone + 'static>(
    func: impl Fn(&T) -> String + 'static,
) -> Arc<dyn DataGridColumnFormatter<T>> {
    Arc::new(FuncFormatter::<T> {
        func: Box::new(func),
    })
}
