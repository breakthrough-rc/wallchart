use crate::{page::PageLayout, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use rscx::html;
use serde::Deserialize;

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
        <PageLayout>
            <h3>New Worker</h3>
            <form hx-post=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id)>
            <div class="form-group">
                <label>First Name</label>
                <input type="text" class="form-control" name="firstName">
            </div>
            <div class="form-group">
                <label>Last Name</label>
                <input type="text" class="form-control" name="lastName">
            </div>
            <button class="btn btn-default">Submit</button>
            </form>
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct ExampleForm {
    foo: String,
    bar: String,
}

async fn post_worker(
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    // Form(example_form): Form<ExampleForm>,
) -> impl IntoResponse {
    println!(
        "wallchart_id: {}, location_id: {}, shift_id: {}",
        wallchart_id, location_id, shift_id
    );

    Html(html! {
        <div>Hi</div>
    })
}
