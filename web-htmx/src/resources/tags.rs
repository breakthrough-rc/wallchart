use crate::{
    components::page::{PageHeader, PageLayout},
    state::WebHtmxState,
};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use rscx::{html, CollectFragmentAsync};
use serde::Deserialize;
use web_client::server::button::PrimaryButton;
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
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Manage Tags".into(),
                buttons: html! {
                    <PrimaryButton
                        onclick="alert('Coming soon!')"
                    >
                        Add Tag
                    </PrimaryButton>
                }
            }
        >
            <p><em>Add, edit, remove tags</em></p>
            <div class="mt-8 flow-root">
                <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                        <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
                            <table class="min-w-full divide-y divide-gray-300">
                                <thead class="bg-gray-50">
                                    <tr>
                                        <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">Tag</th>
                                        <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Icon</th>
                                        <th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
                                            <span class="sr-only">Edit</span>
                                        </th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-gray-200 bg-white">
                                    {
                                        tags.iter().map(|tag| async {
                                            html! {
                                                <tr>
                                                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">{&tag.name}</td>
                                                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{&tag.icon}</td>
                                                    <td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                                                        <a href="#" class="text-indigo-600 hover:text-indigo-900">Edit<span class="sr-only">, Lindsay Walton</span></a>
                                                    </td>
                                                </tr>
                                            }
                                        })
                                        .collect_fragment_async()
                                        .await
                                    }
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
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
