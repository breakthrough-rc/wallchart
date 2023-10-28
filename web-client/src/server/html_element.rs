use super::opt_attrs::opt_attrs;
use rscx::{component, props};
use std::collections::HashMap;

#[props]
pub struct HtmlElementProps {
    #[builder(default)]
    id: String,

    #[builder(default)]
    class: String,

    #[builder(default)]
    role: String,

    #[builder(default)]
    onclick: String,

    #[builder(default)]
    children: String,

    #[builder(default=String::from("div"))]
    tag: String,
}

#[component]
pub fn HtmlElement(props: HtmlElementProps) -> String {
    let attrs = opt_attrs(HashMap::from([
        ("id", props.id),
        ("class", props.class),
        ("role", props.role),
        ("onclick", props.onclick),
    ]));
    let attrs = format!("data-rsx=\"HtmlElement\" {}", attrs);

    format!(
        "<{} {}>{}</{}>",
        props.tag, attrs, props.children, props.tag
    )
}
