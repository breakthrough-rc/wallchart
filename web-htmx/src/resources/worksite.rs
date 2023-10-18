use axum::extract;

pub async fn delete_worker_from_shift(
  extract::Path((
    worksite_id,
    location_id,
    shift_id,
    worker_id,
)): extract::Path<(String, String, String, String)>) -> String {
    println!(
        "Delete worker: {} from shift: {}, from worksite: {} in location: {}", 
        worker_id, 
        shift_id,
        worksite_id,
        location_id,
    );

    "".to_string()
}
