use super::opt_attrs::{opt_attr, opt_attrs};
use rscx::{component, props};
use std::collections::HashMap;

#[props]
pub struct HtmlElementProps {
    #[builder(default)]
    id: String,

    #[builder(default)]
    class: String,

    #[builder(default)]
    onclick: String,

    #[builder(default)]
    role: String,

    #[builder(default)]
    aria_orientation: String,

    #[builder(default)]
    aria_labelledby: String,

    #[builder(default)]
    tabindex: String,

    #[builder(default)]
    data: HashMap<&'static str, String>,

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
        ("aria-orientation", props.aria_orientation),
        ("aria-labelledby", props.aria_labelledby),
        ("tabindex", props.tabindex),
    ]));

    let data_attrs: String = props
        .data
        .into_iter()
        .map(|(key, value)| opt_attr(format!("data-{}", key).as_str(), value))
        .collect::<Vec<String>>()
        .join(" ")
        .to_string();

    let attrs = vec![attrs, data_attrs]
        .into_iter()
        .filter(|attr| !attr.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
        .to_string();

    format!(
        "<{} {}>{}</{}>",
        props.tag, attrs, props.children, props.tag
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rscx::html;

    #[tokio::test]
    async fn test_with_no_attrs() {
        let html = html! {
            <HtmlElement />
        };

        assert_eq!(html, String::from("<div data-rsx=\"HtmlElement\"></div>"));
    }

    #[tokio::test]
    async fn test_with_tag_set() {
        let html = html! {
            <HtmlElement tag="button".into() />
        };

        assert_eq!(
            html,
            String::from("<button data-rsx=\"HtmlElement\"></button>")
        );
    }

    #[tokio::test]
    async fn test_with_children() {
        let html = html! {
            <HtmlElement tag="button".into()>
                <p>Paragraph text.</p>
            </HtmlElement>
        };

        assert_eq!(
            html,
            String::from("<button data-rsx=\"HtmlElement\"><p>Paragraph text.</p></button>")
        );
    }

    #[tokio::test]
    async fn test_with_data_attributes() {
        let html = html! {
            <HtmlElement
                tag="button".into()
                data=HashMap::from([("foo", "baz".into())])
            >
                <h1>Header text.</h1>
            </HtmlElement>
        };

        assert_eq!(
            html,
            String::from(
                "<button data-rsx=\"HtmlElement\" data-foo=\"baz\"><h1>Header text.</h1></button>"
            )
        );
    }
}
