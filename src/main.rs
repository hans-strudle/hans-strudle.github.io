use yew::prelude::*;
mod header;

#[derive(Clone, PartialEq)]
struct Link {
    url: String,
    alt: String,
    img: String,
}

impl Link {
    fn new(url: &str, alt: &str, img: &str) -> Self {
        Link {
            img: img.to_string(),
            url: url.to_string(),
            alt: alt.to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
struct LinksListProps {
    links: Vec<Link>,
}

#[function_component(LinksList)]
fn links_list(LinksListProps { links }: &LinksListProps) -> Html {
    links.iter().map(|link| html! {
        <a href={link.url.clone()}>
            <img src={link.img.clone()} alt={link.alt.clone()} />
        </a>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let links = vec![
        Link::new("https://github.com/hans-strudle","Github Profile", "static/github-mark.png"),
        Link::new("https://linkedin.com/in/strausl","Linkedin Profile","static/github-mark.png"),
    ];

    html! {
        <div>
            <header::Header />
            <LinksList links={links}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
