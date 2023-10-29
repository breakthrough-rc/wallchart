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

    #[builder(default=String::from("HtmlElement"))]
    component_name: String,
}

#[component]
pub fn HtmlElement(props: HtmlElementProps) -> String {
    let attrs = opt_attrs(HashMap::from([
        ("data-rsx", props.component_name),
        ("id", props.id),
        ("class", props.class),
        ("role", props.role),
        ("onclick", props.onclick),
    ]));
    let attrs = attrs.trim().to_string();

    format!(
        "<{} {}>{}</{}>",
        props.tag, attrs, props.children, props.tag
    )
}

#[cfg(test)]
mod tests {
    use rscx::html;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_with_no_attrs() {
        let html = html! {
            <HtmlElement />
        };

        assert_eq!(html, String::from("<div data-rsx=\"HtmlElement\"></div>"));
    }
}
