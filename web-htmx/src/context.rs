use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_login::tower_sessions::Session;
use std::future::Future;

use crate::state::WebHtmxState;

#[derive(Clone)]
pub struct Context {
    pub page_url: String,
    pub worksite_id: String,
    pub worksite_name: String,
}

tokio::task_local! {
    pub(crate) static CONTEXT: Context;
}

pub async fn provide_context_layer(
    State(state): State<WebHtmxState>,
    session: Session,
    request: Request<Body>,
    next: Next,
) -> Response {
    let worksite_id: String = session
        .get("selected_worksite_id")
        .ok()
        .unwrap_or(None)
        .unwrap_or(state.default_worksite_id);

    let worksite_name: String = session
        .get("selected_worksite_name")
        .ok()
        .unwrap_or(None)
        .unwrap_or(state.default_worksite_name);

    let context = Context {
        page_url: request.uri().path().to_string(),
        worksite_id,
        worksite_name,
    };

    // Set the context for this request.
    provide_context(context, next.run(request)).await
}

pub async fn provide_context<F: Future<Output = O>, O>(context: Context, f: F) -> O {
    CONTEXT.scope(context, f).await
}

pub fn context() -> Option<Context> {
    CONTEXT.try_with(|c| c.clone()).ok()
}
