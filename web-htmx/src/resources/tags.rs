use crate::{
    components::{
        page::{PageHeader, PageLayout},
        simple_form::SimpleForm,
    },
    state::WebHtmxState,
};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{delete, get},
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::{component, html, props, CollectFragmentAsync};
use serde::Deserialize;
use web_client::server::{
    button::PrimaryButton,
    form::{GridCell, Label, TextInput},
    modal::Modal,
};
use worksite_service::{add_tag::AddTagInput, get_tag::GetTagInput, get_tags::GetTagsInput};

pub fn tags_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/wallcharts/:worksite_id/tags", get(get_tags))
        .route(
            "/wallcharts/:worksite_id/tags/create-form",
            get(get_create_form).post(post_create_form),
        )
        .route(
            "/wallcharts/:worksite_id/tags/:id/edit-form",
            get(get_edit_form).delete(post_edit_form),
        )
        .route("/wallcharts/:worksite_id/tags/:id", delete(delete_tag))
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
                        hx_get=format!("/wallcharts/{}/tags/create-form", &worksite_id)
                        hx_target="body"
                        hx_swap="beforeend"
                        hx_push_url=format!("/wallcharts/{}/tags/create-form", &worksite_id)
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

async fn get_edit_form(
    extract::Path((worksite_id, tag_id)): extract::Path<(String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let tag = worksite_service
        .get_tag(GetTagInput {
            worksite_id: worksite_id.clone(),
            tag_id,
        })
        .await
        .expect("Failed to get tag")
        .ok_or("Tag not found");

    Html(html! {
        <Modal>
            <TagForm worksite_id=worksite_id />
        </Modal>
    })
}

async fn post_edit_form(
    // extract::Path((worksite_id, tag_id)): extract::Path<(String, String)>,
    State(WebHtmxState { .. }): State<WebHtmxState>,
    Form(_): Form<TagFormData>,
) -> impl IntoResponse {
    todo!()
}

async fn delete_tag(
    extract::Path(_): extract::Path<(String,)>,
    State(_): State<WebHtmxState>,
) -> impl IntoResponse {
    todo!()
}

async fn get_create_form(
    extract::Path(worksite_id): extract::Path<String>,
    State(_): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <Modal>
            <TagForm worksite_id=worksite_id />
        </Modal>
    })
}

async fn post_create_form(
    extract::Path(worksite_id): extract::Path<String>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<TagFormData>,
) -> impl IntoResponse {
    worksite_service
        .add_tag(AddTagInput {
            worksite_id,
            name: form.name,
            icon: form.icon,
        })
        .await
        .expect("Failed to add new tag");

    (
        StatusCode::OK,
        flash.success("Added new tag!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}

#[derive(Deserialize, Debug, Default)]
struct TagFormData {
    name: String,
    icon: String,
}

#[props]
struct TagFormProps {
    worksite_id: String,

    #[builder(default=TagFormData::default())]
    _data: TagFormData, // TODO Support pre-populating form data
}

#[component]
fn TagForm(props: TagFormProps) -> String {
    html! {
        <SimpleForm
            action=format!("/wallcharts/{}/tags/create-form", props.worksite_id)
            description="Add a new tag"
        >
            <GridCell span=6>
                <Label for_input="icon">Icon</Label>
                <TextInput name="icon" />
            </GridCell>
        </SimpleForm>
    }
}
