use crate::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Properties)]
pub struct TredProperties {
    pub k: String,
}

#[derive(Debug, Clone)]
pub enum TredMessage {
    ValueChanged(String),
}

pub struct Tred {
    props: TredProperties,
    link: ComponentLink<Self>,
    value: String,
}

impl Tred {
    fn get_value(&self, props: TredProperties) {
        let link = self.link.clone();
        spawn_local(async move {
            let value = unsafe {
                STRINGS_MAP
                    .as_ref()
                    .and_then(|map| map.get(&props.k).map(|s| s.clone()))
            };
            if let Some(value) = value {
                if value
                    != link
                        .get_component()
                        .map(|t| t.value.clone())
                        .unwrap_or_default()
                {
                    console_log!("key = {}, value = {}", props.k, value);
                    link.send_message(TredMessage::ValueChanged(value));
                }
            }
        });
    }
}

impl Component for Tred {
    type Message = TredMessage;

    type Properties = TredProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            value: String::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TredMessage::ValueChanged(s) => {
                self.value = s;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.get_value(props);
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        self.get_value(self.props.clone());
    }

    fn view(&self) -> Html {
        let dom = web_sys::DomParser::new()
            .unwrap()
            .parse_from_string(
                &format!("<tred>{}</tred>", self.value),
                web_sys::SupportedType::TextHtml,
            )
            .unwrap();
        let body = dom.body().unwrap();
        let nodes = body.children().get_with_index(0).unwrap();
        yew::virtual_dom::VNode::VRef(nodes.into())
    }
}

static mut STRINGS_MAP: Option<HashMap<String, String>> = None;

pub async fn init_strings_map() {
    if let Ok(res) = reqwest::get(base_url().unwrap_or_default() + "/i18n/strings.json").await {
        if let Ok(map) = res.json::<HashMap<String, String>>().await {
            unsafe {
                STRINGS_MAP = Some(map);
            }
        }
    }
}
