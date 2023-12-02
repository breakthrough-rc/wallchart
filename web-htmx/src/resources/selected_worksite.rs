use crate::routes;
use crate::state::WebHtmxState;
use axum::{response::IntoResponse, routing::put, Form, Router};
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

async fn put_selected_worksite(Form(form): Form<SetSelectedWorksiteFormData>) -> impl IntoResponse {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let id = ctx.worksite_id;
    println!("Before setting worksite id {}", id);

    // crate::context::set_worksite_id(form.selected_worksite_id.clone()).await;

    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let id = ctx.worksite_id;
    println!("After setting worksite id {}", id);

    (
        StatusCode::OK,
        [
            ("hx-redirect", routes::worksite(&form.selected_worksite_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}
