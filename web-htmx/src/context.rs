use axum::{http::Request, middleware::Next, response::Response};
use std::future::Future;

#[derive(Clone)]
pub struct Context {
    pub page_url: String,
    pub worksite_id: String,
}

tokio::task_local! {
    pub(crate) static CONTEXT: Context;
}

pub async fn context_provider_layer<B>(request: Request<B>, next: Next<B>) -> Response {
    let context = Context {
        page_url: request.uri().path().to_string(),
        worksite_id: "1".to_string(),
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
