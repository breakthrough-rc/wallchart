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

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "TEST! Hardcode error to test notification!!!",
    )
    // panic!("Hardcode error to test notification!");

    // worksite_service
    //     .remove_worker_from_shift(worksite_id, shift_id, worker_id)
    //     .await
    //     .unwrap();

    // "".to_string()
}
