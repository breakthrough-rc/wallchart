use rscx::{component, html, props};
use web_client::server::{attrs::Attrs, form::Button};

use crate::components::worker_profile_fieldset::WorkerProfileFieldset;

pub struct AddWorkerFormData {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[props]
pub struct AddWorkerFormProps {
    #[builder(setter(into))]
    action: String,
}

#[component]
pub fn AddWorkerForm(props: AddWorkerFormProps) -> String {
    html! {
        <form hx-post=props.action>
            <div class="pb-12">
                <p class="mt-1 text-sm leading-6 text-gray-600">
                    "Please enter the worker's information."
                </p>
                <WorkerProfileFieldset />
            </div>
            <div class="mt-6 flex items-center justify-end gap-x-6">
                <Button
                    onclick="history.go(-1)"
                    attrs=Attrs::with("data-toggle-action", "close".into())
                >
                    Cancel
                </Button>
                <Button kind="submit">Save</Button>
            </div>
        </form>
    }
}
