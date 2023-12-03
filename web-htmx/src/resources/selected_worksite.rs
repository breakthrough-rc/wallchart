use crate::routes;
use crate::state::WebHtmxState;
use axum::{response::IntoResponse, routing::put, Form, Router};
use axum_login::tower_sessions::Session;
use http::StatusCode;
use serde::Deserialize;

pub fn selected_worksite_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::SELECTED_WORKSITE, put(put_selected_worksite))
        .with_state(state)
}

#[derive(Deserialize, Debug)]
struct SetSelectedWorksiteFormData {
    selected_worksite_id: String,
}

async fn put_selected_worksite(
    session: Session,
    Form(form): Form<SetSelectedWorksiteFormData>,
) -> impl IntoResponse {
    session.insert_value(
        "selected_worksite_id",
        form.selected_worksite_id.clone().into(),
    );
    (
        StatusCode::OK,
        [
            ("hx-redirect", routes::worksite(&form.selected_worksite_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}
