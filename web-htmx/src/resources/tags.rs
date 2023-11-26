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
    card::Card,
    form::{GridCell, Label, TextInput},
    headers::SecondaryHeader,
    modal::{modal_target, Modal},
};
use worksite_service::{
    add_tag::AddTagInput, get_tag::GetTagInput, get_tags::GetTagsInput, models::Tag,
    remove_tag::RemoveTagInput, update_tag::UpdateTagInput,
};

use crate::{
    components::{
        page::{PageHeader, PageLayout},
        page_content::PageContent,
        simple_form::{SimpleForm, SimpleFormData},
    },
    routes::{self, tag_edit_form, tags_create_form, TAG, TAGS, TAGS_CREATE_FORM, TAG_EDIT_FORM},
    state::WebHtmxState,
};

pub fn tags_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(TAGS, get(get_tags))
        .route(
            TAGS_CREATE_FORM,
            get(get_create_form).post(post_create_form),
        )
        .route(TAG_EDIT_FORM, get(get_edit_form).post(post_edit_form))
        .route(TAG, delete(delete_tag))
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
                        hx_get=tags_create_form(&worksite_id)
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=tags_create_form(&worksite_id)
                    >
                        Add Tag
                    </PrimaryButton>
                }
            }
        >
            <p><em>Add, edit, remove tags</em></p>
            <PageContent>
                <Card>
                    <TagsTable
                        worksite_id=worksite_id.clone()
                        tags=tags
                    />
                </Card>
            </PageContent>
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
            tag_id: tag_id.clone(),
        })
        .await
        .expect("Failed to get tag")
        .ok_or("Tag not found")
        .expect("Tag not found");

    Html(html! {
        <Modal>
            <SecondaryHeader
                title="🏷️ Edit Tag"
                subtitle="Edit details below."
            />
            <TagForm
                action=tag_edit_form(&worksite_id, &tag_id)
                data=TagFormData {
                    name: tag.name,
                    icon: tag.icon,
                }
            />
        </Modal>
    })
}

async fn post_edit_form(
    extract::Path((worksite_id, tag_id)): extract::Path<(String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<TagFormData>,
) -> impl IntoResponse {
    worksite_service
        .update_tag(UpdateTagInput {
            worksite_id: worksite_id.clone(),
            tag_id: tag_id.clone(),
            name: form.name,
            icon: form.icon,
        })
        .await
        .expect("Failed to add new tag");

    (
        StatusCode::OK,
        flash.success("Tag updated!"),
        [
            ("hx-redirect", routes::tags(&worksite_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}

async fn delete_tag(
    extract::Path((worksite_id, tag_id)): extract::Path<(String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
) -> impl IntoResponse {
    worksite_service
        .remove_tag(RemoveTagInput {
            worksite_id: worksite_id.clone(),
            tag_id: tag_id.clone(),
        })
        .await
        .expect("Failed to remove tag");

    (
        StatusCode::OK,
        flash.success("Tag removed!"),
        [
            ("hx-redirect", routes::tags(&worksite_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}

async fn get_create_form(
    extract::Path(worksite_id): extract::Path<String>,
    State(_): State<WebHtmxState>,
) -> impl IntoResponse {
    Html(html! {
        <Modal>
            <SecondaryHeader
                title="🏷️ Add Tag"
                subtitle="Add a new tag to this worksite."
            />
            <TagForm
                action=tags_create_form(&worksite_id)
            />
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
            worksite_id: worksite_id.clone(),
            name: form.name,
            icon: form.icon,
        })
        .await
        .expect("Failed to add new tag");

    (
        StatusCode::OK,
        flash.success("Added new tag!"),
        [
            ("hx-redirect", routes::tags(&worksite_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[derive(Deserialize, Debug, Default)]
struct TagFormData {
    name: String,
    icon: String,
}

#[props]
struct TagFormProps {
    action: String,

    #[builder(default=TagFormData::default())]
    data: TagFormData,
}

#[component]
fn TagForm(props: TagFormProps) -> String {
    html! {
        <SimpleForm
            action=props.action
            data=SimpleFormData {
                name: props.data.name,
            }
        >
            <GridCell span=6>
                <Label for_input="icon">Icon</Label>
                <TextInput name="icon" value=props.data.icon />
            </GridCell>
        </SimpleForm>
    }
}

#[props]
struct TagsTableProps {
    worksite_id: String,
    tags: Vec<Tag>,
}

#[component]
fn TagsTable(props: TagsTableProps) -> String {
    html! {
        <table class="min-w-full divide-y divide-gray-300">
            <thead class="bg-gray-50">
                <tr>
                    <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">Tag</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Icon</th>
                    <th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
                        <span class="sr-only">Edit</span>
                        <span class="sr-only">Remove</span>
                    </th>
                </tr>
            </thead>
            <tbody class="divide-y divide-gray-200 bg-white">
                {
                    props.tags.iter().map(|tag| async {
                        html! {
                            <tr>
                                <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">{&tag.name}</td>
                                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{&tag.icon}</td>
                                <td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                                    <div class="inline-flex gap-4">
                                        <a
                                            hx-get=tag_edit_form(&props.worksite_id, &tag.id)
                                            hx-target=modal_target()
                                            hx-swap="beforeend"
                                            hx-push-url=tag_edit_form(&props.worksite_id, &tag.id)
                                            class="cursor-pointer text-indigo-600 hover:text-indigo-900"
                                        >
                                            Edit<span class="sr-only">, {&tag.name}</span>
                                        </a>
                                        <a
                                            hx-delete=routes::tag(&props.worksite_id, &tag.id)
                                            hx-confirm="Delete Tag"
                                            data-confirm-message=format!("Are you sure you want to delete tag: {}", &tag.name)
                                            class="cursor-pointer text-indigo-600 hover:text-indigo-900"
                                        >
                                            Remove<span class="sr-only">, {&tag.name}</span>
                                        </a>
                                    </div>
                                </td>
                            </tr>
                        }
                    })
                    .collect_fragment_async()
                    .await
                }
            </tbody>
        </table>
    }
}
