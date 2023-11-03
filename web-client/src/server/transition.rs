#![allow(unused_braces)]
use super::attrs::Attrs;
use super::html_element::HtmlElement;
use rscx::html;
use rscx::{component, props};
use std::collections::HashMap;
use web_macros::html_element;

#[html_element]
pub struct TransitionProps {
    #[builder(setter(into))]
    enter: String,

    #[builder(setter(into))]
    enter_from: String,

    #[builder(setter(into))]
    enter_to: String,

    #[builder(setter(into))]
    leave: String,

    #[builder(setter(into))]
    leave_from: String,

    #[builder(setter(into))]
    leave_to: String,

    #[builder(setter(into))]
    children: String,
}

#[component]
pub fn Transition(props: TransitionProps) -> String {
    let attrs = props.html_attrs_to_hashmap();
    html! {
        <HtmlElement
            tag=props.tag
            class={format!("hidden {}", props.class)}
            component_name="Transition".into()
            data=HashMap::from([
                ("yc-control", "transition".into()),
                ("transition-enter", props.enter),
                ("transition-enter-start", props.enter_from),
                ("transition-enter-end", props.enter_to),
                ("transition-leave", props.leave),
                ("transition-leave-start", props.leave_from),
                ("transition-leave-end", props.leave_to),
            ])
            attrs=Attrs::from(attrs).omit(vec!["class"])
        >
            {props.children}
        </HtmlElement>
    }
}
