use crate::{
    components::{
        login_form::LoginForm,
        page::{PageHeader, PageLayout},
        users::Users,
    },
    state::WebHtmxState,
};
use auth_service::get_user_for_login::GetUserForLoginInput;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use in_memory_user_repository::AuthContext;
use rscx::html;
use serde::Deserialize;
use web_client::server::button::PrimaryButton;

pub fn users_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/login", get(get_login).post(post_login))
        .route("/users", get(get_users))
        .with_state(state)
}

async fn get_login(State(_state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Login">
            <LoginForm login_route="/login" />
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct LoginForm {
    email: String,
    password: String,
}

async fn post_login(
    State(WebHtmxState {
        worksite_service: _,
        auth_service,
        flash_config: _,
    }): State<WebHtmxState>,
    mut auth: AuthContext,
    _flash: Flash,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {
    let result = auth_service
        .get_user_for_login(GetUserForLoginInput {
            email: login_form.email,
            password: login_form.password,
        })
        .await;

    match result {
        Ok(user) => match auth.login(&user).await {
            Ok(_) => (
                StatusCode::OK,
                [("hx-redirect", "/"), ("hx-retarget", "body")],
            )
                .into_response(),
            Err(_) => (StatusCode::BAD_REQUEST, "Login failed").into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Login failed".to_string(),
        )
            .into_response(),
    }
}

async fn get_users(State(state): State<WebHtmxState>) -> impl IntoResponse {
    let users = state
        .auth_service
        .get_users()
        .await
        .expect("Failed to get users");

    Html(html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Users".into(),
                buttons: html! {
                    <PrimaryButton
                        onclick="alert('Coming soon!')"
                    >
                        Add New User
                    </PrimaryButton>
                }
            }
        >
            <Users users=users />
        </PageLayout>
    })
}
