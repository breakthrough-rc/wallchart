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
    button::PrimaryButton,
    card::{Card, CardContent, CardFooter},
    form::{GridCell, GridLayout, Label, TextInput},
    transition::Transition,
    yc_control::Toggle,
};
use worksite_service::{get_assessments::GetAssessmentsInput, models::Assessment};

use crate::{
    components::simple_form::{SimpleForm, SimpleFormData},
    routes,
    state::WebHtmxState,
};

pub fn assessments_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            routes::ASSESSMENTS,
            get(get_assessments).post(post_assessments),
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
                        <div>
                            <h2 id="worker-tags-heading" class="text-lg font-medium leading-6 text-gray-900">"🏅 Assessments"</h2>
                            <p class="mt-1 text-sm text-gray-500">View Assessment History</p>
                        </div>
                        <section class="mt-4 divide-y divide-gray-200 border-b border-t border-gray-200">
                            <AssessmentHistoryList
                                assessments=assessments
                            />
                        </section>
                        <section class="mt-4">
                            <h3 class="text-md mb-2 font-medium leading-6 text-gray-900">"Add a new assessment"</h3>
                            <AssessmentForm />
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

#[derive(Deserialize, Debug)]
struct AssessmentForm {
    value: usize,
    notes: String,
}

async fn post_assessments(
    State(_state): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<AssessmentForm>,
) -> impl IntoResponse {
    println!("form: {:?}", form);

    (
        StatusCode::OK,
        flash.success("New assessment added successfully!"),
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[props]
struct AssessmentHistoryListProps {
    assessments: Vec<Assessment>,
}

#[component]
fn AssessmentHistoryList(props: AssessmentHistoryListProps) -> String {
    html! {
        <ul role="list" class="divide-y divide-gray-100">
            {
                props.assessments.iter().enumerate().map(|(i, assessment)| async move { html! {
                    <li class="flex items-center justify-between gap-x-6 py-5">
                        <div class="min-w-0">
                            <div class="flex items-start gap-x-3">
                                <p class="text-sm font-semibold leading-6 text-gray-900">{format!("Assessment: {}", assessment.value.to_string())}</p>
                                {
                                    if i == 0 {
                                        html! { <p class="rounded-md whitespace-nowrap mt-0.5 px-1.5 py-0.5 text-xs font-medium ring-1 ring-inset text-green-700 bg-green-50 ring-green-600/20">Last Assessment</p> }
                                    } else {
                                        "".into()
                                    }
                                }
                            </div>
                            <div class="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
                                <p class="whitespace-nowrap">"Created "<time datetime="2023-03-17T00:00Z">March 17, 2023</time></p>
                                <svg viewBox="0 0 2 2" class="h-0.5 w-0.5 fill-current">
                                    <circle cx="1" cy="1" r="1" />
                                </svg>
                                <p class="truncate">Assessment by Leslie Alexander</p>
                            </div>
                            <div class="mt-5 flex flex-col gap-x-3">
                                <p class="text-sm font-semibold leading-6 text-gray-900">Notes</p>
                                {
                                    match assessment.notes.is_empty() {
                                        true => html! { <p class="text-xs text-gray-500">No notes.</p> },
                                        false => html! { <p class="text-xs">{&assessment.notes}</p> },
                                    }
                                }
                            </div>
                        </div>
                        <div class="flex flex-none items-center gap-x-4">
                            <PopupMenuButton />
                        </div>
                    </li>
                }})
                .collect_fragment_async()
                .await
            }
        </ul>
    }
}

#[component]
fn PopupMenuButton() -> String {
    html! {
        <Toggle class="relative flex-none">
            <button
                type="button"
                class="-m-2.5 block p-2.5 text-gray-500 hover:text-gray-900"
                id="options-menu-0-button"
                aria-expanded="false"
                aria-haspopup="true"
                data-toggle-action
            >
                <span class="sr-only">Open options</span>
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path d="M10 3a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM10 8.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM11.5 15.5a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z" />
                </svg>
            </button>
            <Transition
                class="absolute hidden right-0 z-10 mt-2 w-32 origin-top-right rounded-md bg-white py-2 shadow-lg ring-1 ring-gray-900/5 focus:outline-none"
                enter="transition ease-out duration-100"
                enter_from="transform opacity-0 scale-95"
                enter_to="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leave_from="transform opacity-100 scale-100"
                leave_to="transform opacity-0 scale-95"
                role="menu"
                aria_orientation="vertical"
                aria_labelledby="options-menu-0-button"
                tabindex="-1"
            >
                // <!-- Active: "bg-gray-50", Not Active: "" -->
                <a href="#" class="block px-3 py-1 text-sm leading-6 text-gray-900" role="menuitem" tabindex="-1" id="options-menu-0-item-0" onclick="alert('Coming soon!')">Edit<span class="sr-only">, Assessment</span></a>
                <a href="#" class="block px-3 py-1 text-sm leading-6 text-gray-900" role="menuitem" tabindex="-1" id="options-menu-0-item-2" onclick="alert('Coming soon!')">Delete<span class="sr-only">, Assessment</span></a>
            </Transition>
        </Toggle>
    }
}

#[props]
struct AssessmentFormProps {}

#[component]
fn AssessmentForm(_props: AssessmentFormProps) -> String {
    html! {
        <GridLayout>
            <GridCell span=6>
                <Label for_input="value">Assessment Value</Label>
                <TextInput name="value" value="" />
            </GridCell>
            <GridCell span=6>
                <Label for_input="notes">Notes</Label>
                <textarea
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    name="notes"
                />
            </GridCell>
        </GridLayout>
    }
}
