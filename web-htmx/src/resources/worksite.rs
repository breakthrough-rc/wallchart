use crate::state::WebHtmxState;
use axum::extract::{self, State};

pub async fn delete_worker_from_shift(
    extract::Path((worksite_id, location_id, shift_id, worker_id)): extract::Path<(
        String,
        String,
        String,
        String,
    )>,
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
) -> String {
    println!(
        "Delete worker: {} from shift: {}, from worksite: {} in location: {}",
        worker_id, shift_id, worksite_id, location_id,
    );

    worksite_service
        .remove_worker_from_shift(worksite_id, shift_id, worker_id)
        .await
        .unwrap();

    "".to_string()
}
