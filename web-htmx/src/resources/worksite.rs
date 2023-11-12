use crate::state::WebHtmxState;
use crate::{components::wallchart::Wallchart, page::PageLayout};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{delete, get},
    Router,
};
use axum_flash::IncomingFlashes;
use http::StatusCode;
use rscx::html;
use web_client::server::notification::NotificationFlashes;
use worksite_service::{
    get_worksite::GetWorksiteInput, remove_worker_from_shift::RemoveWorkerFromShiftInput,
};

pub fn worksite_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/wallchart", get(get_wallchart_page))
        .route(
            "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/:worker_id",
            delete(delete_worker_from_shift),
        )
        .with_state(state)
}

async fn get_wallchart_page(
    flashes: IncomingFlashes,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let worksite = worksite_service
        .get_worksite(GetWorksiteInput {
            id: "1".to_string(),
        })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    let worksite_name = worksite.name.clone();

    let html = html! {
        <PageLayout title=format!("Wallchart: {}", worksite_name)>
            <NotificationFlashes flashes=flashes.clone() />
            <div class="my-4">
                <p><em>Manage your worksite and more.</em></p>
                <Wallchart worksite=worksite/>
            </div>
        </PageLayout>
    };

    (flashes, Html(html))
}

async fn delete_worker_from_shift(
    extract::Path((worksite_id, location_id, shift_id, worker_id)): extract::Path<(
        String,
        String,
        String,
        String,
    )>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    println!(
        "Delete worker: {} from shift: {}, from worksite: {} in location: {}",
        worker_id, shift_id, worksite_id, location_id,
    );

    let result = worksite_service
        .remove_worker_from_shift(RemoveWorkerFromShiftInput {
            id: worksite_id,
            shift_id,
            worker_id,
        })
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
