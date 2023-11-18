use crate::state::WebHtmxState;
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::put,
    Router,
};
use axum_extra::extract::Form as FormExtra;
use axum_flash::Flash;
use http::StatusCode;
use serde::Deserialize;
use worksite_service::assign_tags::AssignTagsInput;

pub fn assigned_tags_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/worksites/:worksite_id/workers/:worker_id/tags",
            put(put_worker_tags),
        )
        .with_state(state)
}

#[derive(Deserialize, Debug)]
struct AssignWorkerTagsFormData {
    #[serde(default)]
    tags: Vec<String>,
}

async fn put_worker_tags(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    FormExtra(form): FormExtra<AssignWorkerTagsFormData>,
) -> impl IntoResponse {
    worksite_service
        .assign_tags(AssignTagsInput {
            worker_id,
            worksite_id,
            tags: form.tags,
        })
        .await
        .expect("Failed to assign tags");

    (
        StatusCode::OK,
        flash.success("Worker tags assigned successfully!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}
