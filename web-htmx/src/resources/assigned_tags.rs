use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{get, put},
    Router,
};
use axum_extra::extract::Form as FormExtra;
use axum_flash::Flash;
use http::StatusCode;
use rscx::{component, html, props, CollectFragmentAsync};
use serde::Deserialize;

use web_client::server::{
    button::PrimaryButton,
    card::{Card, CardContent, CardFooter},
    headers::SecondaryHeader,
};
use worksite_service::{
    assign_tags::AssignTagsInput,
    get_worker::GetWorkerInput,
    get_worksite::GetWorksiteInput,
    models::{Tag, Worker},
};

use crate::{routes, state::WebHtmxState};

pub fn assigned_tags_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::ASSIGNED_TAGS_FORM, get(get_worker_tags_form))
        .route(routes::ASSIGNED_TAGS, put(put_worker_tags))
        .with_state(state)
}

async fn get_worker_tags_form(
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let worksite = state
        .worksite_service
        .get_worksite(GetWorksiteInput {
            id: worksite_id.to_string(),
        })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    let worker = state
        .worksite_service
        .get_worker(GetWorkerInput {
            id: worker_id.clone(),
            worksite_id: worksite_id.clone(),
        })
        .await
        .expect("Failed to get worker")
        .ok_or("Worker not found")
        .expect("Worker not found");

    Html(html! {
        <AssignTagsForm
            action=routes::assigned_tags(&worksite_id, &worker_id)
            tags=worksite.tags
            worker=worker
        />
    })
}

#[derive(Deserialize, Debug)]
struct AssignWorkerTagsFormData {
    #[serde(default)]
    tags: Vec<String>,
}

async fn put_worker_tags(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    FormExtra(form): FormExtra<AssignWorkerTagsFormData>,
) -> impl IntoResponse {
    worksite_service
        .assign_tags(AssignTagsInput {
            worker_id,
            worksite_id,
            tags: form.tags,
        })
        .await
        .expect("Failed to assign tags");

    (
        StatusCode::OK,
        flash.success("Worker tags assigned successfully!"),
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[props]
struct AssignTagsFormProps {
    action: String,
    tags: Vec<Tag>,
    worker: Worker,
}

#[component]
fn AssignTagsForm(props: AssignTagsFormProps) -> String {
    html! {
        <section aria-labelledby="worker-tags-heading">
            <form action="#" method="POST">
                <Card>
                    <CardContent padded=true>
                        <SecondaryHeader
                            id="worker-tags-heading"
                            title="🏷️ Tags"
                            subtitle="Assign tags."
                        />
                        <div class="mt-4 divide-y divide-gray-200 border-b border-t border-gray-200">
                            {
                                #[allow(unused_braces)]
                                props.tags.iter().map(|tag| async {
                                    let tag = tag.clone();
                                    html! {
                                        <div class="relative flex items-start py-4">
                                            <div class="min-w-0 flex-1 text-sm leading-6">
                                                <label for=format!("inp-tag-{}", &tag.id) class="select-none font-medium text-gray-900">{&tag.icon}" "{&tag.name}</label>
                                            </div>
                                            <div class="ml-3 flex h-6 items-center">
                                                <input
                                                    id=format!("inp-tag-{}", &tag.id)
                                                    name="tags"
                                                    type="checkbox"
                                                    class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                                                    { if props.worker.has_tag(&tag) { "checked" } else { "" } }
                                                    value=tag.id.clone()
                                                />
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_fragment_async()
                                .await
                            }
                        </div>
                    </CardContent>
                    <CardFooter>
                        <PrimaryButton
                            hx_put=props.action
                        >
                            Assign Tags
                        </PrimaryButton>
                    </CardFooter>
                </Card>
            </form>
        </section>
    }
}
