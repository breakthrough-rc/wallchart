use crate::state::WebHtmxState;
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

use rscx::{component, props};
use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, TextInput},
};

pub fn locations_routes(state: WebHtmxState) -> Router {
    Router::new()
        // Worksite locations
        .route("/worksites/:worksite_id/locations", post(post_location))
        .route(
            "/worksites/:worksite_id/locations/new-modal",
            get(get_location_form_modal),
        )
        .with_state(state)
}

async fn get_location_form_modal(
    extract::Path(worksite_id): extract::Path<String>,
) -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <LocationForm action=format!("/wallcharts/{}/locations", worksite_id) />
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

#[props]
struct LocationFormProps {
    #[builder(setter(into))]
    action: String,
}

#[component]
fn LocationForm(props: LocationFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.action>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        Add a new location
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=4>
                            <Label for_input="name">Name</Label>
                            <TextInput name="name" />
                        </GridCell>
                        <GridCell span=4>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button
                                    onclick="history.go(-1)"
                                    attrs=Attrs::with("data-toggle-action", "close".into())
                                >
                                    Cancel
                                </Button>
                                <Button kind="submit">Add</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}
