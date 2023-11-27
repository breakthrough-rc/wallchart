use axum::{
    extract::{self, State},
    response::IntoResponse,
    routing::get,
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::{component, html, props, CollectFragmentAsync};
use serde::Deserialize;

use web_client::server::{
    attrs::Attrs,
    button::PrimaryButton,
    card::{Card, CardContent, CardFooter},
    form::{Button, GridCell, GridLayout, Label, TextInput},
    headers::SecondaryHeader,
    modal::{modal_target, Modal},
    popup_menu::{Menu, MenuItem, MenuLink, MenuSize, PopupMenu},
};
use worksite_service::{
    add_assessment::AddAssessmentInput, get_assessment::GetAssessmentInput,
    get_assessments::GetAssessmentsInput, models::Assessment,
    remove_assessment::RemoveAssessmentInput, update_assessment::UpdateAssessmentInput,
};

use crate::{routes, state::WebHtmxState};

pub fn assessments_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            routes::ASSESSMENTS,
            get(get_assessments).post(post_assessments),
        )
        .route(
            routes::ASSESSMENT,
            get(get_assessment_form)
                .put(put_assessment)
                .delete(delete_assessment),
        )
        .with_state(state)
}

async fn get_assessments(
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let assessments = state
        .worksite_service
        .get_assessments(GetAssessmentsInput {
            worksite_id: worksite_id.clone(),
            worker_id: worker_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    html! {
        <section>
            <form>
                <Card>
                    <CardContent padded=true>
                        <SecondaryHeader
                            id="worker-tags-heading"
                            title="🏅 Assessments"
                            subtitle="View Assessment History."
                        />
                        <section class="mt-4 divide-y divide-gray-200 border-b border-t border-gray-200">
                            <AssessmentHistoryList
                                worksite_id=worksite_id.clone()
                                worker_id=worker_id.clone()
                                assessments=assessments
                            />
                        </section>
                        <section class="mt-4">
                            <h3 class="text-md mb-2 font-medium leading-6 text-gray-900">"Add a new assessment"</h3>
                            <AssessmentFormFields />
                        </section>
                    </CardContent>
                    <CardFooter>
                        <PrimaryButton
                            hx_post=routes::assessments(&worksite_id, &worker_id)
                        >
                            Add New Assessment
                        </PrimaryButton>
                    </CardFooter>
                </Card>
            </form>
        </section>
    }
}

#[derive(Deserialize, Debug, Default)]
struct AssessmentFormData {
    value: u8,
    notes: String,
    assessor: String,
}

async fn post_assessments(
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<AssessmentFormData>,
) -> impl IntoResponse {
    worksite_service
        .add_assessment(AddAssessmentInput {
            worker_id: worker_id.clone(),
            worksite_id: worksite_id.clone(),
            value: form.value,
            notes: form.notes,
            assessor: form.assessor,
        })
        .await
        .expect("Failed to add assessment");

    (
        StatusCode::OK,
        flash.success("New assessment added successfully!"),
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
    )
}

async fn get_assessment_form(
    extract::Path((worksite_id, worker_id, assesment_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let assessment = worksite_service
        .get_assessment(GetAssessmentInput {
            worksite_id: worksite_id.clone(),
            worker_id: worker_id.clone(),
            assessment_id: assesment_id.clone(),
        })
        .await
        .expect("Failed to get assessment")
        .ok_or("Tag not found")
        .expect("Tag not found");

    html! {
        <Modal>
            <SecondaryHeader
                title="🏅 Update Assessment"
                subtitle="Enter new values below."
            />
            <div class="mt-4">
                <AssessmentForm
                    action=routes::assessment(&worksite_id, &worker_id, &assesment_id)
                    form_data=AssessmentFormData {
                        value: assessment.value,
                        notes: assessment.notes,
                        assessor: assessment.assessor,
                    }
                />
            </div>
        </Modal>
    }
}

async fn put_assessment(
    extract::Path((worksite_id, worker_id, assessment_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<AssessmentFormData>,
) -> impl IntoResponse {
    worksite_service
        .update_assessment(UpdateAssessmentInput {
            worksite_id,
            worker_id,
            assessment_id,
            value: form.value,
            notes: form.notes,
            assessor: form.assessor,
        })
        .await
        .expect("Failed to update assessment");

    (
        StatusCode::OK,
        flash.success("Assessment updated successfully!"),
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
    )
}

async fn delete_assessment(
    extract::Path((worksite_id, worker_id, assessment_id)): extract::Path<(String, String, String)>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
) -> impl IntoResponse {
    worksite_service
        .remove_assessment(RemoveAssessmentInput {
            worksite_id,
            worker_id,
            assessment_id,
        })
        .await
        .expect("Failed to update assessment");

    (
        StatusCode::OK,
        flash.success("Assessment removed successfully!"),
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[props]
struct AssessmentHistoryListProps {
    worksite_id: String,
    worker_id: String,
    assessments: Vec<Assessment>,
}

#[component]
fn AssessmentHistoryList(props: AssessmentHistoryListProps) -> String {
    let worksite_id = &props.worksite_id;
    let worker_id = &props.worker_id;

    html! {
        <ul role="list" class="divide-y divide-gray-100">
            {
                props.assessments.iter().enumerate().map(|(i, assessment)| async move { html! {
                    <li class="flex items-center justify-between gap-x-6 py-5">
                        <div class="min-w-0">
                            <div class="flex items-start gap-x-3">
                                <p class="text-sm font-semibold leading-6 text-gray-900">{format!("Assessment: {}", assessment.value)}</p>
                                {
                                    if i == 0 {
                                        html! { <p class="rounded-md whitespace-nowrap mt-0.5 px-1.5 py-0.5 text-xs font-medium ring-1 ring-inset text-green-700 bg-green-50 ring-green-600/20">Last Assessment</p> }
                                    } else {
                                        "".into()
                                    }
                                }
                            </div>
                            <div class="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
                                <p class="whitespace-nowrap">
                                {
                                    match assessment.updated_at != assessment.created_at {
                                        true => html! {
                                            <Time
                                                title="Updated"
                                                datetime=assessment.updated_at
                                            />
                                        },
                                        false => html! {
                                            <Time
                                                title="Created"
                                                datetime=assessment.updated_at
                                            />
                                        },
                                    }
                                }
                                </p>
                                <svg viewBox="0 0 2 2" class="h-0.5 w-0.5 fill-current">
                                    <circle cx="1" cy="1" r="1" />
                                </svg>
                                <p class="truncate">{format!("Assessment by {}", &assessment.assessor)}</p>
                            </div>
                            <div class="mt-5 flex flex-col gap-x-3 text-xs ">
                                <p class="font-semibold leading-6 text-gray-900">Notes</p>
                                {
                                    match assessment.notes.is_empty() {
                                        true => html! { <p class="text-gray-500">No notes.</p> },
                                        false => html! { <p>{&assessment.notes}</p> },
                                    }
                                }
                            </div>
                        </div>
                        <div class="flex flex-none items-center gap-x-4">
                            <PopupMenuButton
                                route={
                                    routes::assessment(worksite_id, worker_id, &assessment.id)
                                }
                            />
                        </div>
                    </li>
                }})
                .collect_fragment_async()
                .await
            }
        </ul>
    }
}

#[props]
struct TimeProps {
    #[builder(setter(into))]
    title: String,
    datetime: chrono::DateTime<chrono::Utc>,
}

#[component]
fn Time(props: TimeProps) -> String {
    html! {
        {format!("{} ", props.title)}
        <time datetime=props.datetime.to_rfc3339()>
            {props.datetime.format("%b %e, %Y").to_string()}
        </time>
    }
}

#[props]
struct PopupMenuButtonProps {
    route: String,
}

#[component]
fn PopupMenuButton(props: PopupMenuButtonProps) -> String {
    html! {
        <PopupMenu
            id="assessment-menu"
            class="flex-none"
            size=MenuSize::Small
            button_class="-m-2.5 block p-2.5 text-gray-500 hover:text-gray-900"
            button_content=html! {
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path d="M10 3a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM10 8.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM11.5 15.5a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z" />
                </svg>
            }
        >
            <MenuItem
                title="Edit"
                sr_suffix=", Assessment"
                hx_get=props.route.clone()
                hx_target=modal_target()
                hx_swap="beforeend"
            />
            <MenuItem
                title="Remove"
                sr_suffix=", Assessment"
                hx_get=props.route.clone()
                hx_target=modal_target()
                hx_swap="beforeend"
                hx_confirm="Delete Assessment"
                attrs=Attrs::with(
                    "data-confirm-title", "Are you sure you want to delete this assessment?".into()
                )
            />
        </PopupMenu>
    }
}

#[props]
struct AssessmentFormFieldsProps {
    #[builder(default = AssessmentFormData::default())]
    form_data: AssessmentFormData,
}

#[component]
fn AssessmentFormFields(props: AssessmentFormFieldsProps) -> String {
    html! {
        <GridLayout>
            <GridCell span=6>
                <Label for_input="value">Assessment Value</Label>
                <TextInput name="value" value=&props.form_data.value.to_string() />
            </GridCell>
            <GridCell span=6>
                <Label for_input="assessor">Assessor</Label>
                <TextInput name="assessor" value=&props.form_data.assessor.to_string() />
            </GridCell>
            <GridCell span=6>
                <Label for_input="notes">Notes</Label>
                <textarea
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    name="notes"
                >
                    {&props.form_data.notes}
                </textarea>
            </GridCell>
        </GridLayout>
    }
}

#[props]
struct AssessmentFormProps {
    action: String,

    #[builder(default = AssessmentFormData::default())]
    form_data: AssessmentFormData,
}

#[component]
fn AssessmentForm(props: AssessmentFormProps) -> String {
    html! {
        <form hx-put=props.action>
            <AssessmentFormFields
                form_data=props.form_data
            />
            <GridLayout>
                <GridCell span=6>
                    <div class="mt-6 flex items-center justify-end gap-x-6">
                        <Button
                            onclick="history.go(-1)"
                            attrs=Attrs::with("data-toggle-action", "close".into())
                        >
                            Cancel
                        </Button>
                        <Button kind="submit">Save</Button>
                    </div>
                </GridCell>
            </GridLayout>
        </form>
    }
}
