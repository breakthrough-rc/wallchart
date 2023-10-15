use axum::{response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use web_htmx::{livereload, routes as web_routes};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(web_routes())
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
