use crate::components::{
    add_location_form::AddLocationForm, page::PageLayout, wallchart::Wallchart,
};
use crate::state::WebHtmxState;
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::post,
    routing::{delete, get},
    Form, Router,
};
use axum_flash::{Flash, IncomingFlashes};
use http::StatusCode;
use rscx::html;
use serde::Deserialize;
use web_client::server::{
    modal::{Modal, ModalSize},
    notification::NotificationFlashes,
};
use worksite_service::{
    add_location::AddLocationInput, get_worksite::GetWorksiteInput,
    remove_worker_from_shift::RemoveWorkerFromShiftInput,
};

pub fn worksite_routes(state: WebHtmxState) -> Router {
    Router::new()
        // The actual wallchart
        .route("/wallchart", get(get_wallchart_page))
        // Worksite workers
        .route(
            "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/:worker_id",
            delete(delete_worker_from_shift),
        )
        // Worksite locations
        .route("/wallcharts/:worksite_id/locations", post(post_location))
        .route(
            "/wallcharts/:worksite_id/locations/new-modal",
            get(get_location_form_modal),
        )
        .with_state(state)
}

async fn get_wallchart_page(
    flashes: IncomingFlashes,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let worksite = worksite_service
        .get_worksite(GetWorksiteInput {
            id: "1".to_string(),
        })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    let worksite_name = worksite.name.clone();

    let html = html! {
        <PageLayout header=format!("Wallchart: {}", worksite_name)>
            <NotificationFlashes flashes=flashes.clone() />
            <div class="my-4">
                <Wallchart worksite=worksite/>
            </div>
        </PageLayout>
    };

    (flashes, Html(html))
}

async fn get_location_form_modal(
    extract::Path(worksite_id): extract::Path<String>,
    State(state): State<WebHtmxState>,
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
