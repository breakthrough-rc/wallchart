use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use axum_flash::{Flash, IncomingFlashes};
use http::{HeaderMap, StatusCode};
use rscx::{component, html, props, CollectFragment, CollectFragmentAsync};
use serde::Deserialize;

use web_client::server::{
    attrs::Attrs,
    button::PrimaryButton,
    card::{Card, CardContent, CardFooter},
    flyout::Flyout,
    form::Button,
    headers::SecondaryHeader,
    modal::{modal_target, Modal, ModalSize},
    notification::NotificationFlashes,
};
use worksite_service::{
    add_worker::AddWorkerInput,
    get_worker::GetWorkerInput,
    get_workers::GetWorkersInput,
    get_worksite::GetWorksiteInput,
    models::{Tag, Worker, Worksite},
    update_worker::UpdateWorkerInput,
};

use crate::{
    components::{
        page::{PageHeader, PageLayout},
        page_content::PageContent,
        worker_profile_fieldset::{WorkerProfileFieldset, WorkerProfileFormData},
    },
    routes::{
        self, worker, worker_profile, workers, workers_new, workers_new_modal, WORKER, WORKERS,
        WORKERS_NEW, WORKERS_NEW_MODAL, WORKER_PROFILE,
    },
    state::WebHtmxState,
};

pub fn workers_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(WORKERS, get(get_workers))
        .route(WORKER, get(get_worker_details))
        .route(WORKER_PROFILE, post(post_worker_profile_form))
        .route(WORKERS_NEW, get(get_worker_form).post(post_worker))
        .route(WORKERS_NEW_MODAL, get(get_worker_form_modal))
        .with_state(state)
}

async fn get_workers(
    extract::Path(worksite_id): extract::Path<String>,
    flashes: IncomingFlashes,
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

    let workers = state
        .worksite_service
        .get_workers(GetWorkersInput {
            worksite_id: worksite_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Workers".into(),
                buttons: html! {
                    <PrimaryButton
                        hx_get=workers_new_modal(&worksite_id)
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=workers_new(&worksite_id)
                    >
                        Add New Worker
                    </PrimaryButton>
                }
            }
        >
            <NotificationFlashes flashes=flashes.clone() />
            <PageContent>
                <Card>
                    <WorkersTable worksite=worksite workers=workers/>
                </Card>
            </PageContent>
        </PageLayout>
    })
}

async fn get_worker_details(
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
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

    let full_name = worker.full_name();

    let profile_form_data = WorkerProfileFormData {
        first_name: worker.first_name.clone(),
        last_name: worker.last_name.clone(),
    };

    Html(html! {
        <Flyout title=format!("Worker Detail: {}", &full_name)>
            <div class="w-full border-t border-gray-200 py-6">
                <div class="flex flex-col gap-10">
                    <LoadingWorkerSection
                        route=routes::assigned_tags_form(&worksite_id, &worker_id)
                    />
                    <SectionDivider />
                    <LoadingWorkerSection
                        route=routes::assessments(&worksite_id, &worker_id)
                    />
                    <SectionDivider />
                    <WorkerProfileSection
                        worker_id=worker_id.clone()
                        worksite_id=worksite_id.clone()
                        profile_form_data=profile_form_data
                    />
                </div>
            </div>
        </Flyout>
    })
}

async fn get_worker_form_modal(
    extract::Path(wallchart_id): extract::Path<String>,
) -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <SecondaryHeader
                title="👤 Add Worker"
                subtitle="Add a new worker to this worksite."
            />
            <WorkerForm action=format!("/wallcharts/{}/workers/new", wallchart_id) />
        </Modal>
    })
}

async fn get_worker_form(
    extract::Path(wallchart_id): extract::Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    Html(html! {
        <PageLayout
            partial=headers.contains_key("Hx-Request")
            header="Add Worker"
        >
            <WorkerForm action=workers_new(&wallchart_id) />
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct WorkerFormData {
    pub first_name: String,
    pub last_name: String,
    pub street_address: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
}

async fn post_worker(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path(wallchart_id): extract::Path<String>,
    Form(form): Form<WorkerFormData>,
) -> impl IntoResponse {
    worksite_service
        .add_worker(AddWorkerInput {
            worksite_id: wallchart_id.clone(),
            first_name: form.first_name,
            last_name: form.last_name,
            street_address: form.street_address,
            city: form.city,
            region: form.region,
            postal_code: form.postal_code,
        })
        .await
        .expect("Failed to add worker");

    (
        StatusCode::OK,
        flash.success("Worker added successfully!"),
        [
            ("hx-redirect", workers(&wallchart_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[derive(Deserialize, Debug)]
struct UpdateWorkerFormData {
    first_name: String,
    last_name: String,
    // street_address: String,
    // city: String,
    // region: String,
    // postal_code: String,
}

async fn post_worker_profile_form(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    Form(form): Form<UpdateWorkerFormData>,
) -> impl IntoResponse {
    worksite_service
        .update_worker(UpdateWorkerInput {
            worker_id,
            worksite_id,
            first_name: form.first_name,
            last_name: form.last_name,
        })
        .await
        .expect("Failed to assign worker");

    (
        StatusCode::OK,
        flash.success("Worker updated successfully!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}

#[props]
struct WorkersTableProps {
    worksite: Worksite,
    workers: Vec<Worker>,
}

#[component]
fn WorkersTable(props: WorkersTableProps) -> String {
    html! {
        <table class="min-w-full divide-y divide-gray-300">
            <thead class="bg-gray-50">
                <tr>
                    <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">Name</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Last Assessment</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Tags</th>
                </tr>
            </thead>
            <tbody class="bg-white">
                {
                    props
                        .workers
                        .iter()
                        .map(|worker| async {
                            html! {
                                <WorkerRow
                                    worker=worker.clone()
                                    tags=props.worksite.get_tags_for_worker(worker.clone())
                                />
                            }
                        })
                        .collect_fragment_async()
                        .await
                }
            </tbody>
        </table>
    }
}

#[props]
pub struct WorkerRowProps {
    worker: Worker,
    tags: Vec<Tag>,
}

#[component]
pub fn WorkerRow(props: WorkerRowProps) -> String {
    html! {
        <tr class="border-t border-gray-300" data-loading-states>
            <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">
                  <button
                      hx-get=worker(&"1".to_string(), &props.worker.id)
                      hx-target=modal_target()
                      hx-swap="beforeend"
                  >
                      {format!("{} {}", props.worker.first_name, props.worker.last_name)}
                  </button>
            </td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.worker.last_assessment().map(|assessment| assessment.value).unwrap_or(0)}</td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.tags.into_iter().map(|tag| tag.icon).collect_fragment()}</td>
        </tr>
    }
}

#[props]
struct WorkerFormProps {
    #[builder(setter(into))]
    action: String,
}

#[component]
fn WorkerForm(props: WorkerFormProps) -> String {
    html! {
        <form hx-post=props.action>
            <div class="pb-12">
                <p class="mt-1 text-sm leading-6 text-gray-600">
                    "Please enter the worker's information."
                </p>
                <WorkerProfileFieldset />
            </div>
            <div class="mt-6 flex items-center justify-end gap-x-6">
                <Button
                    onclick="history.go(-1)"
                    attrs=Attrs::with("data-toggle-action", "close".into())
                >
                    Cancel
                </Button>
                <Button kind="submit">Save</Button>
            </div>
        </form>
    }
}

#[props]
struct WorkerProfileSectionProps {
    worksite_id: String,
    worker_id: String,
    profile_form_data: WorkerProfileFormData,
}

#[component]
fn WorkerProfileSection(props: WorkerProfileSectionProps) -> String {
    html! {
        <section aria-labelledby="worker-profile-heading">
            <form>
                <Card>
                    <CardContent padded=true>
                        <SecondaryHeader
                            id="worker-profile-heading"
                            title="👤 Profile"
                            subtitle="Update worker profile details below."
                        />
                        <WorkerProfileFieldset form=props.profile_form_data />
                    </CardContent>
                    <CardFooter>
                        <PrimaryButton
                            hx_post=worker_profile(&props.worksite_id, &props.worker_id)
                        >
                            Update Profile
                        </PrimaryButton>
                    </CardFooter>
                </Card>
            </form>
        </section>
    }
}

#[props]
struct LoadingWorkerSectionProps {
    route: String,
}

#[component]
fn LoadingWorkerSection(props: LoadingWorkerSectionProps) -> String {
    html! {
        <section
            hx-get=props.route
            hx-trigger="revealed"
        >
            <svg class="animate-spin m-auto text-gray-100 h-10 w-10" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
        </section>
    }
}

#[component]
pub fn SectionDivider() -> String {
    html! {
        <div class="relative">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
                <div class="w-full border-t border-gray-300"></div>
            </div>
            <div class="relative flex justify-center">
                <span class="bg-white px-2 text-sm text-gray-500">Continue</span>
            </div>
        </div>
    }
}
