use yew::prelude::*;
/*
#[derive(Clone, PartialEq)]
struct Link {
    url: String,
    alt: String,
}

impl Link {
    fn new(url: &str, alt: &str) -> Self {
        Link {
            url: url.to_string(),
            alt: alt.to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
struct LinksListProps {
    links: Vec<Link>,
}*/

#[function_component(Header)]
pub fn header_div() -> Html {
    html! {
        <div id="header">
            {"Hans"}
        </div>
    }
}
