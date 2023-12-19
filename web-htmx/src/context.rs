use axum::{
    body::Body,
    extract::{Query, State},
    http::Request,
    middleware::Next,
    response::Response,
};
use axum_login::{tower_sessions::Session, AuthSession};
use mongo_user_repository::MongoUserStore;
use std::{collections::HashMap, future::Future};

use auth_service::models::{User, UserPermission, UserRole};

use crate::state::WebHtmxState;

#[derive(Clone)]
pub struct Context {
    pub page_url: String,
    pub page_query_params: HashMap<String, String>,
    pub worksite_id: String,
    pub worksite_name: String,
    pub current_user: Option<LoggedInUser>,
    pub is_partial_request: bool,
}

#[derive(Clone)]
pub struct LoggedInUser {
    pub id: String,
    pub email: String,
    pub role: UserRole,

    user: User,
}

impl LoggedInUser {
    pub fn has_perm(&self, perm: UserPermission) -> bool {
        self.user.has_perm(perm)
    }
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
    let Query(query_params): Query<HashMap<String, String>> =
        Query::try_from_uri(request.uri()).unwrap();

    let is_partial_request = request.headers().contains_key("Hx-Request");

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
            id: user.id.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            user,
        }),
        None => None,
    };

    println!("email: {:?}", current_user.clone().unwrap().email);
    println!(
        "current_user role: {:?}",
        current_user.clone().unwrap().role
    );

    let context = Context {
        page_url: request.uri().path().to_string(),
        page_query_params: query_params,
        worksite_id,
        worksite_name,
        current_user,
        is_partial_request,
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
