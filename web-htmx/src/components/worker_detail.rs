use rscx::{component, html, props};
use worksite_service::models::Worker;

use super::add_worker_form::AddWorkerForm;

#[props]
pub struct WorkerDetailProps {
    #[builder(setter(into))]
    worker: Worker,
}

#[component]
pub fn WorkerDetail(props: WorkerDetailProps) -> String {
    html! {
        <div>
           <AddWorkerForm action="" first_name=props.worker.first_name last_name=props.worker.last_name />
        </div>
    }
}
