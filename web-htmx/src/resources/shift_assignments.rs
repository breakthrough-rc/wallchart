use crate::{
    components::{assign_shift_form::AssignShiftForm, page::PageLayout},
    routes,
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
    assign_worker::AssignWorkerInput, get_workers::GetWorkersInput,
    remove_worker_from_shift::RemoveWorkerFromShiftInput,
};

pub fn shift_assignments_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            routes::SHIFT_ASSIGNMENTS_NEW,
            get(get_shift_assignment_form).post(post_shift_assignment),
        )
        .route(
            routes::SHIFT_ASSIGNMENTS_NEW_MODAL,
            get(get_shift_assignment_form_modal),
        )
        .route(routes::SHIFT_ASSIGNMENT, delete(delete_worker_from_shift))
        .with_state(state)
}

async fn delete_worker_from_shift(
    extract::Path((worksite_id, _location_id, shift_id, worker_id)): extract::Path<(
        String,
        String,
        String,
        String,
    )>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
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
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let workers = state
        .worksite_service
        .get_workers(GetWorkersInput {
            worksite_id: wallchart_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <AssignShiftForm
                workers=workers
                action=routes::shift_assignments_new(&wallchart_id, &location_id, &shift_id)
                create_worker_action=routes::workers_new(&wallchart_id)
            />
        </Modal>
    })
}

async fn get_shift_assignment_form(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let workers = state
        .worksite_service
        .get_workers(GetWorkersInput {
            worksite_id: wallchart_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <PageLayout header="Assign Shift">
            <AssignShiftForm
                workers=workers
                action=routes::shift_assignments_new(&wallchart_id, &location_id, &shift_id)
                create_worker_action=routes::workers_new(&wallchart_id)
            />
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
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
    )
}
