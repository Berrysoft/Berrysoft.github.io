use crate::*;

pub fn parse_html(s: &str) -> Html {
    let dom = web_sys::DomParser::new()
        .unwrap()
        .parse_from_string(
            &format!("<parse>{}</parse>", s),
            web_sys::SupportedType::TextHtml,
        )
        .unwrap();
    let body = dom.body().unwrap();
    yew::virtual_dom::VNode::VRef(body.children().get_with_index(0).unwrap().into())
}
