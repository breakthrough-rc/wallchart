use crate::routes::LOGIN;
use crate::state::WebHtmxState;
use crate::{components::page::PageLayout, routes};
use auth_service::get_user_for_login::GetUserForLoginInput;
use axum::extract::Query;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use axum_login::AuthSession;
use http::StatusCode;
use mongo_user_repository::MongoUserStore;
use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::form::{Button, GridCell, GridLayout, Label, TextInput};

pub fn login_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::LOGIN, get(get_login).post(post_login))
        .route(routes::LOGOUT, post(post_logout))
        .route(routes::FORBIDDEN, get(get_forbidden))
        .with_state(state)
}

async fn get_login(Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Login">
            <LoginForm login_route=routes::login() next=next/>
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct LoginForm {
    email: String,
    password: String,
    next: Option<String>,
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

async fn post_login(
    State(WebHtmxState { auth_service, .. }): State<WebHtmxState>,
    mut auth: AuthSession<MongoUserStore>,
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
            Ok(_) => {
                let next = match login_form.next {
                    Some(next) => next,
                    None => routes::home(),
                };
                (
                    StatusCode::OK,
                    [("hx-redirect", next), ("hx-retarget", "body".to_string())],
                )
                    .into_response()
            }
            Err(_) => (StatusCode::BAD_REQUEST, "Login failed").into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Login failed".to_string(),
        )
            .into_response(),
    }
}

#[props]
struct LoginFormProps {
    #[builder(setter(into))]
    login_route: String,

    #[builder(setter(into))]
    next: Option<String>,
}

#[component]
fn LoginForm(props: LoginFormProps) -> String {
    html! {
        <form hx-post=props.login_route>
            <div class="pb-12">
                <p class="mt-1 text-sm leading-6 text-gray-600">
                    "pssst: try user@yallchart.com / password"
                </p>
                <GridLayout class="mt-10">
                    <GridCell span=4>
                        <Label for_input="email">Email</Label>
                        <TextInput input_type="email" name="email" autocomplete="email" />
                    </GridCell>
                    <GridCell span=4>
                        <Label for_input="password">Password</Label>
                        <TextInput input_type="password" name="password" autocomplete="password" />
                    </GridCell>
                    <GridCell span=4>
                        <div class="mt-6 flex items-center justify-end gap-x-6">
                            <Button kind="submit">Login</Button>
                        </div>
                    </GridCell>
                </GridLayout>
            </div>
            {
                match props.next {
                    Some(next) => html! {
                        <input type="hidden" name="next" value=next />
                    },
                    None => html! {},
                }
            }
        </form>
    }
}

async fn post_logout(mut auth: AuthSession<MongoUserStore>) -> impl IntoResponse {
    match auth.logout() {
        Ok(_) => (
            StatusCode::OK,
            [("hx-redirect", LOGIN), ("hx-retarget", "body")],
        )
            .into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Login failed").into_response(),
    }
}

async fn get_forbidden() -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Access Denied">
            <p>"You are not authorized to view this page"</p>
        </PageLayout>
    })
}
