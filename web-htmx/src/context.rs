use axum::{extract::State, http::Request, middleware::Next, response::Response};
use std::future::Future;

use crate::state::WebHtmxState;

#[derive(Clone)]
pub struct Context {
    pub page_url: String,
    pub worksite_id: String,
}

tokio::task_local! {
    pub(crate) static CONTEXT: Context;
}

pub async fn provide_context_layer<B>(
    State(state): State<WebHtmxState>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let context = Context {
        page_url: request.uri().path().to_string(),
        worksite_id: state.default_worksite_id.clone(),
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
