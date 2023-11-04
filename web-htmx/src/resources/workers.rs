use crate::{page::PageLayout, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use rscx::html;
use serde::Deserialize;
use web_client::server::form::{Button, Label, TextInput};

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
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <PageLayout title="Add Worker">
            <form hx-post=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id)>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        "Please enter the worker's information."
                    </p>
                    <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                        <div class="sm:col-span-3">
                            <Label for_input="first_name">First name</Label>
                            <div class="mt-2">
                                <TextInput name="first_name" autocomplete="given-name" />
                            </div>
                        </div>
                        <div class="sm:col-span-3">
                            <Label for_input="last_name">Last name</Label>
                            <div class="mt-2">
                                <TextInput name="last_name" autocomplete="family-name" />
                            </div>
                        </div>
                        <div class="sm:col-span-4">
                            <Label for_input="email">Email address</Label>
                            <div class="mt-2">
                                <TextInput input_type="email" name="email" autocomplete="email" />
                            </div>
                        </div>
                        <div class="sm:col-span-3">
                            <Label for_input="country">Country</Label>
                            <div class="mt-2">
                                <select id="country" name="country" autocomplete="country-name" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:max-w-xs sm:text-sm sm:leading-6">
                                    <option default>United States</option>
                                    <option>Canada</option>
                                    <option>Mexico</option>
                                </select>
                            </div>
                        </div>
                        <div class="col-span-full">
                            <Label for_input="street_address">Street address</Label>
                            <div class="mt-2">
                                <TextInput name="street_address" autocomplete="street-address" />
                            </div>
                        </div>
                        <div class="sm:col-span-2 sm:col-start-1">
                            <Label for_input="city">City</Label>
                            <div class="mt-2">
                                <TextInput name="city" autocomplete="address-level2" />
                            </div>
                        </div>
                        <div class="sm:col-span-2">
                            <Label for_input="region">State</Label>
                            <div class="mt-2">
                                <TextInput name="region" autocomplete="address-level1" />
                            </div>
                        </div>
                        <div class="sm:col-span-2">
                            <Label for_input="postal_code">Zip Code</Label>
                            <div class="mt-2">
                                <TextInput name="postal_code" autocomplete="postal-code" />
                            </div>
                        </div>
                    </div>
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
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    Form(form): Form<AddWorkerForm>,
) -> impl IntoResponse {
    println!(
        "wallchart_id: {}, location_id: {}, shift_id: {}",
        wallchart_id, location_id, shift_id
    );

    println!("add_worker: {:?}", form);

    Html(html! {
        <div>Hi</div>
    })
}
