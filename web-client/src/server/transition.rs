#![allow(unused_braces)]
use super::html_element::HtmlElement;
use rscx::html;
use rscx::{component, props};
use std::collections::HashMap;

#[props]
#[derive(Debug)]
pub struct TransitionProps {
    #[builder(default)]
    enter: String,

    #[builder(default)]
    enter_from: String,

    #[builder(default)]
    enter_to: String,

    #[builder(default)]
    leave: String,

    #[builder(default)]
    leave_from: String,

    #[builder(default)]
    leave_to: String,

    #[builder(default)]
    class: String,

    #[builder(default)]
    role: String,

    #[builder(default)]
    aria_orientation: String,

    #[builder(default)]
    aria_labelledby: String,

    #[builder(default)]
    tabindex: String,

    #[builder(default)]
    children: String,

    #[builder(default=String::from("div"))]
    tag: String,
}

#[component]
pub fn Transition(props: TransitionProps) -> String {
    html! {
        <HtmlElement
            tag=props.tag
            class={format!("hidden {}", props.class)}
            component_name="Transition".into()
            aria_orientation=props.aria_orientation
            aria_labelledby=props.aria_labelledby
            role=props.role
            tabindex=props.tabindex
            data=HashMap::from([
                ("yc-control", "transition".into()),
                ("transition-enter", props.enter),
                ("transition-enter-start", props.enter_from),
                ("transition-enter-end", props.enter_to),
                ("transition-leave", props.leave),
                ("transition-leave-start", props.leave_from),
                ("transition-leave-end", props.leave_to),
            ])
        >
            {props.children}
        </HtmlElement>
    }
}
