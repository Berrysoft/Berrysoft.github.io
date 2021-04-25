use crate::*;
use yew::virtual_dom::VNode;

pub fn parse_html(s: &str) -> Html {
    let div = yew::utils::document().create_element("div").unwrap();
    div.set_inner_html(s);
    VNode::VRef(div.into())
}
