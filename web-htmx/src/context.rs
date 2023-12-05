use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_login::{
    tower_sessions::{MongoDBStore, Session},
    AuthSession,
};
use mongo_user_repository::MongoUserStore;
use std::future::Future;

use crate::state::WebHtmxState;

#[derive(Clone)]
pub struct Context {
    pub page_url: String,
    pub worksite_id: String,
    pub worksite_name: String,
    pub logged_in: bool,
    pub current_user: Option<LoggedInUser>,
}

#[derive(Clone)]

pub struct LoggedInUser {
    pub id: String,
    pub email: String,
    pub role: String,
}

tokio::task_local! {
    pub(crate) static CONTEXT: Context;
}

pub async fn provide_context_layer(
    State(state): State<WebHtmxState>,
    session: Session,
    auth: AuthSession<MongoUserStore>,
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

    let current_user = match auth.user {
        Some(user) => Some(LoggedInUser {
            id: user.id.into(),
            email: user.email.into(),
            role: "superadmin".into(), // TODO: get this from the user
        }),
        None => None,
    };

    let context = Context {
        page_url: request.uri().path().to_string(),
        worksite_id,
        worksite_name,
        current_user,
        logged_in: session.get_value("x.logged.in.user").is_some(),
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
