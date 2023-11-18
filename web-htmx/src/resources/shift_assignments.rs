use crate::{
    components::{
        add_worker_form::AddWorkerForm, assign_shift_form::AssignShiftForm, page::PageLayout,
    },
    state::WebHtmxState,
};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{delete, get},
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::html;
use serde::Deserialize;
use web_client::server::modal::{Modal, ModalSize};
use worksite_service::{
    assign_worker::AssignWorkerInput, remove_worker_from_shift::RemoveWorkerFromShiftInput,
};

pub fn shift_assignments_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new",
            get(get_shift_assignment_form).post(post_shift_assignment),
        )
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new-modal",
            get(get_shift_assignment_form_modal),
        )
        .route(
            "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/:worker_id",
            delete(delete_worker_from_shift),
        )
        .with_state(state)
}

async fn delete_worker_from_shift(
    extract::Path((worksite_id, location_id, shift_id, worker_id)): extract::Path<(
        String,
        String,
        String,
        String,
    )>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    println!(
        "Delete worker: {} from shift: {}, from worksite: {} in location: {}",
        worker_id, shift_id, worksite_id, location_id,
    );

    let result = worksite_service
        .remove_worker_from_shift(RemoveWorkerFromShiftInput {
            id: worksite_id,
            shift_id,
            worker_id,
        })
        .await;

    match result {
        Ok(_) => "".into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error deleting worker from shift",
        )
            .into_response(),
    }
}

async fn get_shift_assignment_form_modal(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState { .. }): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <AssignShiftForm workers=vec![] action=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id) />
        </Modal>
    })
}

async fn get_shift_assignment_form(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState { .. }): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Assign Shift">
            <AssignShiftForm workers=vec![] action=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id) />
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct AssignShiftFormData {
    worker_id: String,
}

async fn post_shift_assignment(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    Form(form): Form<AssignShiftFormData>,
) -> impl IntoResponse {
    worksite_service
        .assign_worker(AssignWorkerInput {
            worksite_id: wallchart_id,
            location_id,
            shift_id,
            worker_id: form.worker_id,
        })
        .await
        .expect("Failed to assign worker");

    (
        StatusCode::OK,
        flash.success("Shift assigned successfully!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}
