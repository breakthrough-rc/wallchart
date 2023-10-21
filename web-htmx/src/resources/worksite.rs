use crate::state::WebHtmxState;
use axum::{
    extract::{self, State},
    response::IntoResponse,
};
use http::StatusCode;

pub async fn delete_worker_from_shift(
    extract::Path((worksite_id, location_id, shift_id, worker_id)): extract::Path<(
        String,
        String,
        String,
        String,
    )>,
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
) -> impl IntoResponse {
    println!(
        "Delete worker: {} from shift: {}, from worksite: {} in location: {}",
        worker_id, shift_id, worksite_id, location_id,
    );

    let result = worksite_service
        .remove_worker_from_shift(worksite_id, shift_id, worker_id)
        .await;

    match result {
        Ok(_) => "".into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error deleting worker from shift",
        )
            .into_response(),
    }
}
