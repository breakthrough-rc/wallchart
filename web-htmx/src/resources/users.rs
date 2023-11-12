use crate::{components::login_form::LoginForm, page::PageLayout, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use rscx::html;
use serde::Deserialize;

pub fn users_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/login", get(get_login))
        .with_state(state)
}

async fn get_login(State(state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout title="Login">
            <LoginForm login_route="/login" />
        </PageLayout>
    })
}

// #[derive(Deserialize, Debug)]
// struct ExampleForm {
//     foo: String,
//     bar: String,
// }
//
// async fn post_users(
//     State(WebHtmxState {
//         worksite_service,
//         auth_service,
//         flash_config,
//     }): State<WebHtmxState>,
//     Form(example_form): Form<ExampleForm>,
// ) -> impl IntoResponse {
//     todo!()
// }
