use crate::{components::add_location_form::AddLocationForm, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::html;
use serde::Deserialize;
use web_client::server::modal::{Modal, ModalSize};
use worksite_service::add_location::AddLocationInput;

pub fn locations_routes(state: WebHtmxState) -> Router {
    Router::new()
        // Worksite locations
        .route("/wallcharts/:worksite_id/locations", post(post_location))
        .route(
            "/wallcharts/:worksite_id/locations/new-modal",
            get(get_location_form_modal),
        )
        .with_state(state)
}

async fn get_location_form_modal(
    extract::Path(worksite_id): extract::Path<String>,
) -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <AddLocationForm action=format!("/wallcharts/{}/locations", worksite_id) />
        </Modal>
    })
}

#[derive(Deserialize, Debug)]
struct AddLocationFormData {
    name: String,
}

async fn post_location(
    extract::Path(worksite_id): extract::Path<String>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<AddLocationFormData>,
) -> impl IntoResponse {
    worksite_service
        .add_location(AddLocationInput {
            worksite_id,
            location_name: form.name,
        })
        .await
        .expect("Failed to add new location");

    (
        StatusCode::OK,
        flash.success("Added new location!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}
