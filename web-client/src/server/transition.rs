use super::html_element::HtmlElement;
use rscx::{component, html, props};
use std::collections::HashMap;
use web_macros::*;

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
    html! {
        <HtmlElement
            tag=props.tag
            class=format!("hidden {}", props.class)
            component_name="Transition"
            data=HashMap::from([
                ("yc-control", "transition".into()),
                ("transition-enter", props.enter),
                ("transition-enter-start", props.enter_from),
                ("transition-enter-end", props.enter_to),
                ("transition-leave", props.leave),
                ("transition-leave-start", props.leave_from),
                ("transition-leave-end", props.leave_to),
            ])
            attrs=spread_attrs!(props | omit(class))
        >
            {props.children}
        </HtmlElement>
    }
}
