use std::sync::Arc;

use auth_service::RequireAuth;
use axum::{
    response::{Html, Redirect},
    routing::get,
    Router,
};
use http::StatusCode;
use page::PageLayout;
//##PLOP USE RESOURCE HOOK##
use components::not_found_message::NotFoundMessage;
use resources::users::users_routes;
use resources::workers::workers_routes;
use resources::worksite::worksite_routes;
use rscx::html;
use state::WebHtmxState;
use web_client::routes as client_routes;

pub mod components;
pub mod livereload;
pub mod page;
pub mod playground;
pub mod resources;
pub mod state;

pub fn routes(state: WebHtmxState) -> Router {
    Router::new()
        .with_state(state.clone())
        //##PLOP MERGE ROUTE HOOK##
        .merge(worksite_routes(state.clone()))
        .merge(workers_routes(state.clone()))
        // Anything above this RequireAuth route will require authentication
        // .route_layer(RequireAuth::login_or_redirect(
        //     Arc::new("/login".into()),
        //     None,
        // ))
        .route("/", get(Redirect::temporary("/playground")))
        .nest("/playground", playground::routes())
        .nest_service("/client", client_routes())
        .merge(users_routes(state.clone()))
        .fallback(fallback)
}

async fn fallback() -> (StatusCode, Html<String>) {
    let not_found = html! {
        <PageLayout title="Oops!">
            <NotFoundMessage />
        </PageLayout>
    };

    (StatusCode::NOT_FOUND, Html(not_found))
}
