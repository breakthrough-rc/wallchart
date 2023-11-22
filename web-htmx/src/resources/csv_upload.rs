use crate::{routes, state::WebHtmxState};
use axum::{
    extract::{State},
    response::{Html, IntoResponse},
    routing::get, Router,
};
use rscx::html;


pub fn csv_upload_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::CSV_UPLOAD, get(get_csv_upload))
        .with_state(state)
}

async fn get_csv_upload(State(_state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <p>"CSV Upload"</p>
    })
}
