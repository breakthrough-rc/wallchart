use crate::{page::PageLayout, resources::worksite, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use http::StatusCode;
use rscx::html;
use serde::Deserialize;
use web_client::server::form::{Button, CellSpan, GridCell, GridLayout, Label, Select, TextInput};
use worksite_service::assign_worker::AssignWorkerInput;

pub fn workers_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new",
            get(get_worker_form).post(post_worker),
        )
        .with_state(state)
}

async fn get_worker_form(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <PageLayout title="Add Worker">
            <form hx-post=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id)>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        "Please enter the worker's information."
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=3>
                            <Label for_input="last_name">First name</Label>
                            <TextInput name="first_name" autocomplete="given-name" />
                        </GridCell>

                        <GridCell span=3>
                            <Label for_input="last_name">Last name</Label>
                            <TextInput name="last_name" autocomplete="family-name" />
                        </GridCell>

                        <GridCell span=4>
                            <Label for_input="email">Email address</Label>
                            <TextInput input_type="email" name="email" autocomplete="email" />
                        </GridCell>

                        <GridCell span=3>
                            <Label for_input="country">Country</Label>
                            <Select id="country" name="country" autocomplete="country-name">
                                <option default>United States</option>
                                <option>Canada</option>
                                <option>Mexico</option>
                            </Select>
                        </GridCell>

                        <GridCell span=CellSpan::Full>
                            <Label for_input="street_address">Street address</Label>
                            <TextInput name="street_address" autocomplete="street-address" />
                        </GridCell>

                        <GridCell span=2 start=1>
                            <Label for_input="city">City</Label>
                            <TextInput name="city" autocomplete="address-level2" />
                        </GridCell>

                        <GridCell span=2>
                            <Label for_input="region">State</Label>
                            <TextInput name="region" autocomplete="address-level1" />
                        </GridCell>

                        <GridCell span=2>
                            <Label for_input="postal_code">Zip Code</Label>
                            <TextInput name="postal_code" autocomplete="postal-code" />
                        </GridCell>
                    </GridLayout>
                </div>
                <div class="mt-6 flex items-center justify-end gap-x-6">
                    <Button onclick="history.go(-1)">Cancel</Button>
                    <Button kind="submit">Save</Button>
                </div>
            </form>
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct AddWorkerForm {
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
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    Form(form): Form<AddWorkerForm>,
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
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}
