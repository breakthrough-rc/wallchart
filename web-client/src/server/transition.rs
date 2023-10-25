use rscx::html;
use rscx::{component, props};

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
}

#[component]
pub fn Transition(props: TransitionProps) -> String {
    html! {
        // TODO! Support tags other than div as root element.
        <div
            class={format!("hidden {}", props.class)}
            data-transition-enter={props.enter}
            data-transition-enter-start={props.enter_from}
            data-transition-enter-end={props.enter_to}
            data-transition-leave={props.leave}
            data-transition-leave-start={props.leave_from}
            data-transition-leave-end={props.leave_to}
            // TODO! Do not render role and aria attributes if they are empty.
            role=props.role
            aria-orientation=props.aria_orientation
            aria-labelledby=props.aria_labelledby
            tabindex=props.tabindex
        >
            {props.children}
        </div>
    }
}
