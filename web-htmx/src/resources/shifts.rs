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
use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::modal::{Modal, ModalSize};
use worksite_service::add_shift::AddShiftInput;

use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, TextInput},
};

pub fn shifts_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/worksites/:worksite_id/locations/:location_id/shifts",
            post(post_shifts),
        )
        .route(
            "/worksites/:worksite_id/locations/:location_id/shifts/new-modal",
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
            <ShiftForm action=format!("/worksites/{}/locations/{}/shifts", worksite_id, location_id) />
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

#[props]
struct ShiftFormProps {
    #[builder(setter(into))]
    action: String,
}

#[component]
fn ShiftForm(props: ShiftFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.action>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        Add a new shift
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
