use rscx::{component, html, props};
use worksite_service::models::Worker;

#[props]
pub struct AssignShiftFormProps {
    #[builder(setter(into))]
    workers: Vec<Worker>,

    #[builder(setter(into))]
    action: String,
}

#[component]
pub fn AssignShiftForm(props: AssignShiftFormProps) -> String {
    html! {
        <div>
            <div>Assign workers form</div>
        </div>
    }
}
