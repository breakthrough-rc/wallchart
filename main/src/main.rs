use axum::{response::IntoResponse, routing::get, Router};
use in_memory_worksite_repository::InMemoryWorksiteRepository;
use std::{net::SocketAddr, sync::Arc};
use usecases::{
    get_worksite::GetWorksite,
    ports::worksite_repository,
    remove_worker_from_shift::{self, RemoveWorkerFromShift},
    service::WorksiteService,
};
use web_htmx::{livereload, routes as web_routes, state::WebHtmxState};

#[tokio::main]
async fn main() {
    // Create worksite service
    let worksite_repository = Arc::new(InMemoryWorksiteRepository::empty());
    let get_worksite = GetWorksite {
        worksite_repository: worksite_repository.clone(),
    };
    let remove_worker_from_shift = RemoveWorkerFromShift {
        worksite_repository,
    };
    let worksite_service = WorksiteService {
        get_worksite,
        remove_worker_from_shift,
    };

    // Create WebHtmxState
    let web_htmx_state = WebHtmxState {
        worksite_service: Arc::new(worksite_service),
    };

    let app = Router::new()
        .merge(web_routes(web_htmx_state))
        .route("/healthcheck", get(get_health_check));

    #[cfg(debug_assertions)]
    let app = app.layer(livereload::layer());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn get_health_check() -> impl IntoResponse {
    "ONE SMALL STEP FOR AN ASSHOLE, ONE GIANT LEAP FOR ASSHOLEKIND"
}
