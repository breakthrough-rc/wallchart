use rscx::{component, html, props};
use worksite_service::models::Worker;

use super::add_worker_form::AddWorkerForm;

#[props]
pub struct WorkerDetailProps {
    #[builder(setter(into))]
    worker: Worker,
}

#[component]
pub fn WorkerDetail(_props: WorkerDetailProps) -> String {
    html! {
        <div>
           <AddWorkerForm action=""/>
        </div>
    }
}
