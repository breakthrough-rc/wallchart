use crate::{components::simple_form::SimpleForm, routes, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::{headers::SecondaryHeader, modal::Modal};
use worksite_service::add_location::AddLocationInput;

pub fn locations_routes(state: WebHtmxState) -> Router {
    Router::new()
        // Worksite locations
        .route(routes::LOCATIONS, post(post_location))
        .route(routes::LOCATIONS_CREATE_FORM, get(get_location_form_modal))
        .with_state(state)
}

async fn get_location_form_modal(
    extract::Path(worksite_id): extract::Path<String>,
) -> impl IntoResponse {
    Html(html! {
        <Modal>
            <SecondaryHeader
                title="Add Location"
                subtitle="Add a new location to this worksite."
            />
            <LocationForm action=routes::locations(&worksite_id) />
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
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[props]
struct LocationFormProps {
    #[builder(setter(into))]
    action: String,
}

#[component]
fn LocationForm(props: LocationFormProps) -> String {
    html! {
        <SimpleForm
            action=props.action
        />
    }
}
