use crate::{components::page::PageLayout, state::WebHtmxState};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use rscx::{html, CollectFragmentAsync};
use serde::Deserialize;
use worksite_service::get_tags::GetTagsInput;

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

async fn get_tags(
    extract::Path(worksite_id): extract::Path<String>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let tags = state
        .worksite_service
        .get_tags(GetTagsInput {
            worksite_id: worksite_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <PageLayout header="Manage Tags">
            {
                tags.iter().map(|tag| async {
                    html! {
                        <div>{&tag.name}</div>
                    }
                })
                .collect_fragment_async()
                .await
            }
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
