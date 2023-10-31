#![allow(unused_braces)]
use super::html_element::{html_attrs, Attrs, HtmlElement};
use rscx::html;
use rscx::{component, props};
use std::collections::HashMap;

html_attrs! {
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
        children: String,
    }
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
