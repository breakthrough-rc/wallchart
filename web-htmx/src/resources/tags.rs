use crate::{components::page::PageLayout, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use rscx::html;
use serde::Deserialize;

pub fn tags_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/wallcharts/:worksite_id/tags",
            get(get_tags).post(post_tags),
        )
        .route(
            "/wallcharts/:worksite_id/tags/:id",
            get(get_tags_detail).delete(delete_tags),
        )
        .with_state(state)
}

async fn get_tags(State(_): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Manage Tags">
            <p>Managing tags... coming soon!</p>
        </PageLayout>
    })
}

async fn get_tags_detail(
    extract::Path(_): extract::Path<(String,)>,
    State(_): State<WebHtmxState>,
) -> impl IntoResponse {
    todo!()
}

#[derive(Deserialize, Debug)]
struct ExampleForm {}

async fn post_tags(State(_): State<WebHtmxState>, Form(_): Form<ExampleForm>) -> impl IntoResponse {
    todo!()
}

async fn delete_tags(
    extract::Path(_): extract::Path<(String,)>,
    State(_): State<WebHtmxState>,
) -> impl IntoResponse {
    todo!()
}
