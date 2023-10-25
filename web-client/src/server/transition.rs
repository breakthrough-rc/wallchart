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
    children: String,
}

#[component]
pub fn Transition(props: TransitionProps) -> String {
    // Currently transition component only supports div as root element.
    html! {
        <div
            class={format!("hidden {}", props.class)}
            data-transition-enter={props.enter}
            data-transition-enter-start={props.enter_from}
            data-transition-enter-end={props.enter_to}
            data-transition-leave={props.leave}
            data-transition-leave-start={props.leave_from}
            data-transition-leave-end={props.leave_to}
        >
            {props.children}
        </div>
    }
}
