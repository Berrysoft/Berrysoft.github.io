use crate::*;
use std::fmt::Debug;

pub trait DataGridItem {
    fn prop(&self, name: &str) -> &dyn DataGridItemProperty;
}

pub trait DataGridItemProperty {
    fn fmt_html(&self) -> Html;
}

impl DataGridItemProperty for String {
    fn fmt_html(&self) -> Html {
        html! {{self}}
    }
}

#[derive(Debug)]
pub struct DataGrid<T: DataGridItem + Clone + 'static> {
    props: DataGridProperties<T>,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Properties)]
pub struct DataGridProperties<T: DataGridItem + Clone + 'static> {
    pub children: ChildrenWithProps<DataGridColumn>,
    pub data: Rc<Vec<T>>,
}

impl<T: DataGridItem + Clone + 'static> Component for DataGrid<T> {
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
        log::debug!("DataGrid view: {}", self.props.data.len());
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
pub struct DataGridColumn {
    props: DataGridColumnProperties,
}

#[derive(Debug, Clone, Properties)]
pub struct DataGridColumnProperties {
    pub header: String,
    pub prop: String,
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
        html! {
            <th scope="col">{&self.props.header}</th>
        }
    }
}
