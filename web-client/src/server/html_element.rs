use super::opt_attrs::{opt_attr, opt_attrs};
use crate::html_attrs;
use rscx::{component, props};
use std::collections::HashMap;

html_attrs! {
    pub struct HtmlElementProps {
        #[builder(default)]
        children: String,

        #[builder(default=String::from("HtmlElement"))]
        component_name: String,
    }
}

#[component]
pub fn HtmlElement(props: HtmlElementProps) -> String {
    let attrs = opt_attrs(
        HashMap::from([("data-rsx", props.component_name.clone())])
            .into_iter()
            .chain(props.html_attrs_to_hashmap())
            .collect::<HashMap<&str, String>>(),
    );

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
    use crate::server::attrs::Attrs;
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

    #[tokio::test]
    async fn test_with_attrs_with_omit() {
        // Emulate usage by a component
        // Common use case: we want to apply the `class` attribute (or any attribute) manually
        // Then pass the right of the props, omitting `class`.
        let built_props = HtmlElementProps::builder();
        let outer_props = built_props
            .id("set-id".into())
            .role("set-role".into())
            .class("THIS_CLASS_SHOULD_BE_OMITTED".into())
            .build();

        let html = html! {
            <HtmlElement
                class="hard-coded-class".into()
                attrs=Attrs::from(outer_props).omit(vec!["class"])
            >
                What an awesome element!
            </HtmlElement>
        };

        assert!(
            html.contains("class=\"hard-coded-class\""),
            "The class attr is set from rscx NOT overwritten by props."
        );
        assert!(
            !html.contains("THIS_CLASS_SHOULD_BE_OMITTED"),
            "The class attr should have omitted class prop."
        );
        assert!(
            html.contains("id=\"set-id\"") && html.contains("role=\"set-role\""),
            "Contains id and role attrs from props."
        );
    }
}
