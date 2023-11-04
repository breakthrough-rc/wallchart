use super::html_element::HtmlElement;
use rscx::{component, html, props};
use web_macros::*;

#[html_element]
pub struct LabelProps {
    #[builder(setter(into))]
    for_input: String,
    children: String,
}

#[component]
pub fn Label(props: LabelProps) -> String {
    html! {
        <HtmlElement
            tag="label"
            class=format!("block text-sm font-medium leading-6 text-gray-900 {}", props.class).trim()
            attrs=spread_attrs!(props | omit(class)).set("for", props.for_input)
        >
            {props.children}
        </HtmlElement>
    }
}
