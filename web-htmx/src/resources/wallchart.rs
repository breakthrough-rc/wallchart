use crate::state::WebHtmxState;
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Form, Router,
};
use serde::Deserialize;

pub fn wallchart_routes(state: WebHtmxState) {
    Router::new()
        .route("/wallchart", get(get_wallcharts).post(post_wallcharts))
        .route("/wallchart/:id", get(get_wallchart).delete(delete_wallchart))
        .with_state(state)
}

async fn get_wallcharts(
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
) -> impl IntoResponse {
  todo!()
}

async fn get_wallchart(
    extract::Path((id)): extract::Path<(
        String,
    )>,
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
) -> impl IntoResponse {
  todo!()
}

#[derive(Deserialize, Debug)]
struct ExampleForm {
    foo: String,
    bar: String,
}

async fn post_wallcharts(
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
    Form(example_form): Form<ExampleForm>,
) -> impl IntoResponse {
  todo!()
}

async fn delete_wallchart(
    extract::Path((id)): extract::Path<(
        String,
    )>,
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
) -> impl IntoResponse {
  todo!()
}
