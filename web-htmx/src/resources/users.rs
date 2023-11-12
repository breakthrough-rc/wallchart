use crate::state::WebHtmxState;
use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Form, Router,
};
use serde::Deserialize;

pub fn users_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/users", get(get_users).post(post_users))
        .route("/users/:id", get(get_users).delete(delete_users))
        .with_state(state)
}

async fn get_users(
    State(WebHtmxState {
        worksite_service,
        auth_service,
        flash_config,
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    todo!()
}

async fn get_users_detail(
    extract::Path((id)): extract::Path<(String,)>,
    State(WebHtmxState {
        worksite_service,
        auth_service,
        flash_config,
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    todo!()
}

#[derive(Deserialize, Debug)]
struct ExampleForm {
    foo: String,
    bar: String,
}

async fn post_users(
    State(WebHtmxState {
        worksite_service,
        auth_service,
        flash_config,
    }): State<WebHtmxState>,
    Form(example_form): Form<ExampleForm>,
) -> impl IntoResponse {
    todo!()
}

async fn delete_users(
    extract::Path((id)): extract::Path<(String,)>,
    State(WebHtmxState {
        worksite_service,
        auth_service,
        flash_config,
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    todo!()
}
