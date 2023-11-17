use crate::{
    components::{
        add_worker_form::AddWorkerForm,
        page::{PageHeader, PageLayout},
        worker_detail::WorkerDetail,
        worker_profile_fieldset::{WorkerProfileFieldset, WorkerProfileFormData},
        workers::Workers,
    },
    state::WebHtmxState,
};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Form, Router,
};
use axum_flash::{Flash, IncomingFlashes};
use http::StatusCode;
use rscx::html;
use serde::Deserialize;
use web_client::server::{
    button::PrimaryButton,
    flyout::Flyout,
    modal::{Modal, ModalSize},
    notification::NotificationFlashes,
};
use worksite_service::{
    add_worker::AddWorkerInput, get_worker::GetWorkerInput, get_workers::GetWorkersInput,
    get_worksite::GetWorksiteInput, update_worker::UpdateWorkerInput,
};

pub fn workers_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/worksites/:worksite_id/workers", get(get_workers))
        .route(
            "/worksites/:worksite_id/workers/:worker_id",
            get(get_worker_details),
        )
        .route(
            "/worksites/:worksite_id/workers/profile/:worker_id",
            get(get_worker_profile_form).post(post_worker_profile_form),
        )
        .route("/workers", get(Redirect::temporary("/worksites/1/workers")))
        .route(
            "/wallcharts/:worksite_id/workers/new",
            get(get_worker_form).post(post_worker),
        )
        .route(
            "/wallcharts/:worksite_id/workers/new-modal",
            get(get_worker_form_modal),
        )
        .with_state(state)
}

async fn get_workers(
    extract::Path(worksite_id): extract::Path<String>,
    flashes: IncomingFlashes,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let worksite = state
        .worksite_service
        .get_worksite(GetWorksiteInput {
            id: worksite_id.to_string(),
        })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    let workers = state
        .worksite_service
        .get_workers(GetWorkersInput {
            worksite_id: worksite_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Workers".into(),
                buttons: html! {
                    <PrimaryButton
                        hx_get=format!("/wallcharts/{}/workers/new-modal", &worksite_id)
                        hx_target="body"
                        hx_swap="beforeend"
                        hx_push_url=format!("/wallcharts/{}/workers/new", &worksite_id)
                    >
                        Add New Worker
                    </PrimaryButton>
                }
            }
        >
            <NotificationFlashes flashes=flashes.clone() />
            <div class="my-4">
                <div class="mt-8 flow-root">
                    <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                        <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                            <Workers worksite=worksite workers=workers/>
                        </div>
                    </div>
                </div>
            </div>
        </PageLayout>
    })
}

async fn get_worker_details(
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let worker = state
        .worksite_service
        .get_worker(GetWorkerInput {
            id: worker_id.clone(),
            worksite_id: worksite_id.clone(),
        })
        .await
        .expect("Failed to get worker")
        .ok_or("Worker not found")
        .expect("Worker not found");

    let full_name = worker.full_name();

    let profile_form_data = WorkerProfileFormData {
        first_name: worker.first_name,
        last_name: worker.last_name,
    };

    Html(html! {
        <Flyout title=format!("Worker Detail: {}", &full_name)>
            <section aria-labelledby="payment-details-heading">
                <form action="#" method="POST">
                    <div class="shadow sm:overflow-hidden sm:rounded-md">
                        <div class="bg-white px-4 py-6 sm:p-6">
                            <div>
                                <h2 id="payment-details-heading" class="text-lg font-medium leading-6 text-gray-900">Worker Profile</h2>
                                <p class="mt-1 text-sm text-gray-500">Update worker profile details below.</p>
                            </div>
                            <WorkerProfileFieldset form=profile_form_data />
                        </div>
                        <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
                            <PrimaryButton
                                hx_post=format!("/worksites/{}/workers/profile/{}", &worksite_id, &worker_id)
                            >
                                Save
                            </PrimaryButton>
                        </div>
                    </div>
                </form>
            </section>
        </Flyout>
    })
}
// <div class="space-y-6 sm:px-6 lg:col-span-9 lg:px-0">
// <div class="overflow-hidden rounded-md bg-white px-6 py-4 shadow">
//     <div class="border-b border-gray-200 bg-white px-4 py-5 sm:px-6">
//         <div class="-ml-4 -mt-2 flex flex-wrap items-center justify-between sm:flex-nowrap">
//             <div class="ml-4 mt-2">
//                 <h3 class="text-base font-semibold leading-6 text-gray-900">Job Postings</h3>
//             </div>
//             <div class="ml-4 mt-2 flex-shrink-0">
//                 <button type="button" class="relative inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Remove Worker</button>
//             </div>
//         </div>
//     </div>
//     <WorkerDetail worker=worker />
// </div>

async fn get_worker_profile_form(
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let worker = state
        .worksite_service
        .get_worker(GetWorkerInput {
            id: worker_id,
            worksite_id,
        })
        .await
        .expect("Failed to get worker")
        .ok_or("Worker not found")
        .expect("Worker not found");

    let full_name = worker.full_name();

    Html(html! {
        <Flyout title=format!("Worker Detail: {}", full_name)>
            <WorkerDetail worker=worker />
        </Flyout>
    })
}

async fn get_worker_form_modal(
    extract::Path(wallchart_id): extract::Path<String>,
) -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <AddWorkerForm action=format!("/wallcharts/{}/workers/new", wallchart_id) />
        </Modal>
    })
}

async fn get_worker_form(extract::Path(wallchart_id): extract::Path<String>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Add Worker">
            <AddWorkerForm action=format!("/wallcharts/{}/workers/new", wallchart_id) />
        </PageLayout>
    })
}

async fn post_worker(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path(wallchart_id): extract::Path<String>,
    Form(form): Form<AddWorkerFormData>,
) -> impl IntoResponse {
    worksite_service
        .add_worker(AddWorkerInput {
            worksite_id: wallchart_id.clone(),
            first_name: form.first_name,
            last_name: form.last_name,
            street_address: form.street_address,
            city: form.city,
            region: form.region,
            postal_code: form.postal_code,
        })
        .await
        .expect("Failed to add worker");

    (
        StatusCode::OK,
        flash.success("Worker added successfully!"),
        [
            (
                "hx-redirect",
                format!("/worksites/{}/workers", wallchart_id),
            ),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[derive(Deserialize, Debug)]
struct AddWorkerFormData {
    first_name: String,
    last_name: String,
    street_address: String,
    city: String,
    region: String,
    postal_code: String,
}

#[derive(Deserialize, Debug)]
struct UpdateWorkerFormData {
    first_name: String,
    last_name: String,
    // street_address: String,
    // city: String,
    // region: String,
    // postal_code: String,
}

async fn post_worker_profile_form(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    Form(form): Form<UpdateWorkerFormData>,
) -> impl IntoResponse {
    worksite_service
        .update_worker(UpdateWorkerInput {
            worker_id,
            worksite_id,
            first_name: form.first_name,
            last_name: form.last_name,
        })
        .await
        .expect("Failed to assign worker");

    (
        StatusCode::OK,
        flash.success("Worker updated successfully!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}
