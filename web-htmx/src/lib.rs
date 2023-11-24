use axum::{
    response::{Html, Redirect},
    routing::get,
    Router,
};
use http::StatusCode;
//##PLOP USE RESOURCE HOOK##
use components::{not_found_message::NotFoundMessage, page::PageLayout};
use resources::assigned_tags::assigned_tags_routes;
use resources::csv_upload::csv_upload_routes;
use resources::locations::locations_routes;
use resources::shift_assignments::shift_assignments_routes;
use resources::shifts::shifts_routes;
use resources::tags::tags_routes;
use resources::users::users_routes;
use resources::workers::workers_routes;
use resources::worksite::worksite_routes;
use routes::{CLIENT, HOME, PLAYGROUND};
use rscx::html;
use state::WebHtmxState;
use web_client::routes as client_routes;

pub mod components;
pub mod livereload;
pub mod playground;
pub mod resources;
mod routes;
pub mod state;

#[cfg(debug_assertions)]
const HOME_REDIRECT: &str = PLAYGROUND;

#[cfg(not(debug_assertions))]
const HOME_REDIRECT: &'static str = WALLCHART;

pub fn routes(state: WebHtmxState) -> Router {
    Router::new()
        .with_state(state.clone())
        //##PLOP MERGE ROUTE HOOK##
        .merge(csv_upload_routes(state.clone()))
        .merge(shift_assignments_routes(state.clone()))
        .merge(shifts_routes(state.clone()))
        .merge(locations_routes(state.clone()))
        .merge(tags_routes(state.clone()))
        .merge(worksite_routes(state.clone()))
        .merge(workers_routes(state.clone()))
        .merge(assigned_tags_routes(state.clone()))
        // Anything above this RequireAuth route will require authentication
        // .route_layer(RequireAuth::login_or_redirect(
        //     Arc::new(LOGIN.into()),
        //     None,
        // ))
        .route(HOME, get(Redirect::temporary(HOME_REDIRECT)))
        .nest(PLAYGROUND, playground::routes())
        .nest_service(CLIENT, client_routes())
        .merge(users_routes(state))
        .fallback(fallback)
}

async fn fallback() -> (StatusCode, Html<String>) {
    let not_found = html! {
        <PageLayout header="Oops!">
            <NotFoundMessage />
        </PageLayout>
    };

    (StatusCode::NOT_FOUND, Html(not_found))
}
