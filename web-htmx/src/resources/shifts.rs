use crate::components::add_shift_form::AddShiftForm;
use crate::state::WebHtmxState;
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    routing::post,
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::html;
use serde::Deserialize;
use web_client::server::modal::{Modal, ModalSize};
use worksite_service::add_shift::AddShiftInput;

pub fn shifts_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts",
            post(post_shifts),
        )
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/new-modal",
            get(get_shift_form_modal),
        )
        .with_state(state)
}

async fn get_shift_form_modal(
    extract::Path((worksite_id, location_id)): extract::Path<(String, String)>,
    State(_): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <AddShiftForm action=format!("/wallcharts/{}/locations/{}/shifts", worksite_id, location_id) />
        </Modal>
    })
}

#[derive(Deserialize, Debug)]
struct AddShiftFormData {
    name: String,
}

async fn post_shifts(
    extract::Path((worksite_id, location_id)): extract::Path<(String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<AddShiftFormData>,
) -> impl IntoResponse {
    worksite_service
        .add_shift(AddShiftInput {
            worksite_id,
            location_id,
            shift_name: form.name,
        })
        .await
        .expect("Failed to add new shift");

    (
        StatusCode::OK,
        flash.success("Added new shift!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}
