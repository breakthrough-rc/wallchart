use axum::{
    extract::{State},
    response::IntoResponse,
    routing::get,
    Form, Router,
};
use rscx::html;
use serde::Deserialize;

use crate::routes;
use crate::state::WebHtmxState;

pub fn assessments_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            routes::ASSESSMENTS,
            get(get_assessments).post(post_assessments),
        )
        .with_state(state)
}

async fn get_assessments(State(_state): State<WebHtmxState>) -> impl IntoResponse {
    html! {
        <h1>Assessments</h1>
        <p>Coming Soon</p>
    }
}

#[derive(Deserialize, Debug)]
struct ExampleForm {
    foo: String,
    bar: String,
}

async fn post_assessments(
    State(_state): State<WebHtmxState>,
    Form(_example_form): Form<ExampleForm>,
) -> impl IntoResponse {
    todo!()
}
