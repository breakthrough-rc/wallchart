use crate::{
    components::{add_worker_form::AddWorkerForm, worker_detail::WorkerDetail},
    page::PageLayout,
    state::WebHtmxState,
};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::html;
use serde::Deserialize;
use web_client::server::modal::{Modal, ModalSize};
use worksite_service::{
    assign_worker::AssignWorkerInput, get_worker::GetWorkerInput,
    update_worker::UpdateWorkerInput,
};

pub fn workers_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/worksites/:worksite_id/workers/:worker_id",
            get(get_worker_detail).post(post_worker_detail),
        )
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new",
            get(get_worker_form).post(post_worker),
        )
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new-modal",
            get(get_worker_form_modal),
        )
        .with_state(state)
}

async fn get_worker_detail(
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
        <PageLayout
            title=format!("Worker Detail: {}", full_name)
        >
            <WorkerDetail worker=worker />
        </PageLayout>
    })
}

async fn get_worker_form_modal(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState { .. }): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <AddWorkerForm action=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id) />
        </Modal>
    })
}

async fn get_worker_form(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState { .. }): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <PageLayout title="Add Worker">
            <AddWorkerForm action=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id) />
        </PageLayout>
    })
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

async fn post_worker(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    Form(form): Form<AddWorkerFormData>,
) -> impl IntoResponse {
    println!(
        "wallchart_id: {}, location_id: {}, shift_id: {}",
        wallchart_id, location_id, shift_id
    );

    println!("add_worker: {:?}", form);

    worksite_service
        .assign_worker(AssignWorkerInput {
            id: wallchart_id,
            location_id,
            shift_id,
            first_name: form.first_name,
            last_name: form.last_name,
            street_address: form.street_address,
            city: form.city,
            region: form.region,
            postal_code: form.postal_code,
        })
        .await
        .expect("Failed to assign worker");

    (
        StatusCode::OK,
        flash.success("Worker added successfully!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}

#[derive(Deserialize, Debug)]
struct UpdateWorkerFormData {
    first_name: String,
    last_name: String,
    street_address: String,
    city: String,
    region: String,
    postal_code: String,
}

async fn post_worker_detail(
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
