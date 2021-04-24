use crate::*;
use std::iter::FromIterator;
use web_sys::*;
use yew::virtual_dom::VNode;

pub fn parse_html(s: &str) -> Html {
    let dom = DomParser::new()
        .unwrap()
        .parse_from_string(s, SupportedType::TextHtml)
        .unwrap();
    let body = dom.body().unwrap();
    let nodes = body.child_nodes();
    VNode::from_iter((0..nodes.length()).map(|i| VNode::VRef(nodes.get(i).unwrap())))
}
