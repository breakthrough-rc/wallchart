use crate::{routes, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use rscx::html;
use serde::Deserialize;

pub fn csv_upload_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::CSV_UPLOAD, get(get_csv_upload))
        .with_state(state)
}

async fn get_csv_upload(State(state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <p>"CSV Upload"</p>
    })
}
