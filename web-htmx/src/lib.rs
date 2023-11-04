use axum::{
    response::{Html, Redirect},
    routing::{delete, get},
    Router,
};
use http::StatusCode;
use page::PageLayout;
use pages::wallchart::get_wallchart_page;
//##PLOP USE RESOURCE HOOK##
use components::not_found_message::NotFoundMessage;
use resources::workers::workers_routes;
use resources::worksite::worksite_routes;
use rscx::html;
use state::WebHtmxState;
use web_client::routes as client_routes;

pub mod components;
pub mod livereload;
pub mod page;
pub mod pages;
pub mod playground;
pub mod resources;
pub mod state;

pub fn routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/", get(Redirect::temporary("/playground")))
        .route("/wallchart", get(get_wallchart_page))
        .nest_service("/client", client_routes())
        .with_state(state.clone())
        .merge(worksite_routes(state.clone()))
        .merge(workers_routes(state))
        .merge(playground::routes())
        .fallback(fallback)
    //##PLOP MERGE ROUTE HOOK##
}

async fn fallback() -> (StatusCode, Html<String>) {
    let not_found = html! {
        <PageLayout title="Oops!">
            <NotFoundMessage />
        </PageLayout>
    };

    (StatusCode::NOT_FOUND, Html(not_found))
}
