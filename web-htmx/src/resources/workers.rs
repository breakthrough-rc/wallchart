use std::collections::HashMap;

use axum::{
    extract::{self, State},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use axum_flash::{Flash, IncomingFlashes};
use axum_macros::debug_handler;
use futures::future::join_all;
use http::{HeaderMap, StatusCode};
use rscx::{component, html, props, CollectFragment};
use serde::Deserialize;
use validator::{Validate, ValidationErrorsKind};

use web_client::server::{
    attrs::Attrs,
    button::PrimaryButton,
    card::{Card, CardContent, CardFooter},
    flyout::Flyout,
    form::{Button, GridCell, TextInput},
    headers::SecondaryHeader,
    modal::{modal_target, Modal, ModalSize},
    notification::NotificationFlashes,
    table::{TDVariant, Table, TableData, TableHeading},
};
use worksite_service::{
    add_worker::AddWorkerInput,
    filter_workers::FilterWorkersInput,
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
        .route(WORKERS, post(filter_workers))
        .route(WORKER, get(get_worker_details))
        .route(WORKER_PROFILE, post(post_worker_profile_form))
        .route(WORKERS_NEW, get(get_worker_form).post(post_worker))
        .route(WORKERS_NEW_MODAL, get(get_worker_form_modal))
        .with_state(state)
}

#[derive(Deserialize)]
struct FilterWorkersFormData {
    filter: String,
}

#[debug_handler]
async fn filter_workers(
    State(state): State<WebHtmxState>,
    extract::Path(worksite_id): extract::Path<String>,
    Form(form_data): Form<FilterWorkersFormData>,
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
        .filter_workers(FilterWorkersInput {
            worksite_id: worksite_id.clone(),
            filter: form_data.filter,
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <Card>
            <WorkersTable worksite=worksite workers=workers/>
        </Card>
    })
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

    let worksite_name = worksite.name.clone();

    Html(html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Workers".into(),
                buttons: html! {
                    <GridCell>
                        <TextInput class="form-control py-1.5" input_type="search"
                            name="filter" placeholder="Search..."
                            hx_post=routes::workers(&worksite_id)
                            hx_trigger="input changed delay:500ms, filter"
                            hx_target="table">
                        </TextInput>
                    </GridCell>
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
            <PageContent title=format!("Manage all workers for {}", worksite_name)>
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
        email: worker.email.clone(),
        street_address: worker
            .address
            .clone()
            .map(|a| a.street_address)
            .unwrap_or("".into()),
        city: worker.address.clone().map(|a| a.city).unwrap_or("".into()),
        region: worker
            .address
            .clone()
            .map(|a| a.region)
            .unwrap_or("".into()),
        postal_code: worker
            .address
            .clone()
            .map(|a| a.postal_code)
            .unwrap_or("".into()),
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
            <WorkerForm action=workers_new(&wallchart_id) />
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

async fn post_worker(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path(wallchart_id): extract::Path<String>,
    Form(form): Form<WorkerProfileFormData>,
) -> Response {
    if let Err(e) = form.validate() {
        let profile_form_data = WorkerProfileFormData {
            first_name: form.first_name.clone(),
            last_name: form.last_name.clone(),
            email: form.email.clone(),
            street_address: form.street_address.clone(),
            city: form.city.clone(),
            region: form.region.clone(),
            postal_code: form.postal_code.clone(),
        };

        return (
            StatusCode::BAD_REQUEST,
            Html(html! {
                <Alert
                    title="Oops! There was a problem with your submission."
                />
                <WorkerForm
                    action=workers_new(&wallchart_id)
                    form_data=profile_form_data
                    errors=e.errors().to_owned()
                />
            }),
        )
            .into_response();
    }

    worksite_service
        .add_worker(AddWorkerInput {
            worksite_id: wallchart_id.clone(),
            first_name: form.first_name,
            last_name: form.last_name,
            email: form.email,
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
        .into_response()
}

async fn post_worker_profile_form(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((worksite_id, worker_id)): extract::Path<(String, String)>,
    Form(form): Form<WorkerProfileFormData>,
) -> impl IntoResponse {
    worksite_service
        .update_worker(UpdateWorkerInput {
            worker_id,
            worksite_id,
            first_name: form.first_name,
            last_name: form.last_name,
            email: form.email,
            street_address: form.street_address,
            city: form.city,
            region: form.region,
            postal_code: form.postal_code,
        })
        .await
        .expect("Failed to update worker");

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
        <Table
            headings=vec![
                TableHeading::title("Name"),
                TableHeading::title("Last Assessment"),
                TableHeading::title("Tags"),
            ]
            body=join_all(props
                .workers
                .iter()
                .map(|worker| async {
                    html! {
                        <WorkerTableRow
                            worksite_id=props.worksite.id.clone()
                            worker=worker.clone()
                            tags=props.worksite.get_tags_for_worker(worker.clone())
                        />
                    }
                }))
                .await
        />
    }
}

#[props]
pub struct WorkerTableRowProps {
    worksite_id: String,
    worker: Worker,
    tags: Vec<Tag>,
}

#[component]
pub fn WorkerTableRow(props: WorkerTableRowProps) -> String {
    html! {
        <TableData variant=TDVariant::First>
            <button
                hx-get=worker(&props.worksite_id, &props.worker.id)
                hx-target=modal_target()
                hx-swap="beforeend"
            >
                {format!("{} {}", props.worker.first_name, props.worker.last_name)}
            </button>
        </TableData>
        <TableData>
            {props.worker.last_assessment().map(|assessment| assessment.value).unwrap_or(0)}
        </TableData>
        <TableData variant=TDVariant::LastNonEmptyHeading>
            {
                props.tags
                    .into_iter()
                    .map(|tag| html! { <span title=tag.name class="cursor-pointer">{tag.icon}</span> })
                    .collect_fragment()
            }
        </TableData>
    }
}

#[props]
struct WorkerFormProps {
    #[builder(setter(into))]
    action: String,

    #[builder(default=WorkerProfileFormData::default())]
    form_data: WorkerProfileFormData,

    #[builder(default=HashMap::new())]
    errors: HashMap<&'static str, ValidationErrorsKind>,
}

#[component]
fn WorkerForm(props: WorkerFormProps) -> String {
    html! {
        <form
            hx-ext="response-targets"
            id="form-worker"
            hx-post=props.action
            hx-target-4xx="this"
        >
            <div class="pb-12">
                <WorkerProfileFieldset
                    form=props.form_data
                    errors=props.errors
                />
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

// TODO: Move this to a shared component

#[props]
struct AlertProps {
    #[builder(setter(into))]
    title: String,

    #[builder(default)]
    children: String,
}

#[component]
fn Alert(props: AlertProps) -> String {
    html! {
        <div class="mt-6 rounded-md bg-red-50 p-4">
            <div class="flex">
                <div class="flex-shrink-0">
                    <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z" clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="ml-3">
                    <h3 class="text-sm font-medium text-red-800">{props.title}</h3>
                    {if props.children.is_empty() {
                        String::new()
                    } else {
                        html! { <div class="mt-2 text-sm text-red-700">{props.children}</div> }
                    }}
                </div>
            </div>
        </div>
    }
}
