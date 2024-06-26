use axum::{
    extract::{self, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_flash::{Flash, IncomingFlashes};
use axum_login::tower_sessions::Session;
use axum_macros::debug_handler;
use http::StatusCode;
use rscx::{
    component, html, props, typed_builder::TypedBuilder, CollectFragment, CollectFragmentAsync,
};
use serde::Deserialize;

use auth_service::models::UserPermission;
use web_client::server::{
    attrs::Attrs,
    button::{PrimaryButton, SecondaryButton},
    card::Card,
    form::{Button, GridCell, GridLayout, Label, TextInput},
    headers::SecondaryHeader,
    modal::{modal_target, Modal, ModalSize},
    notification::NotificationFlashes,
};
use worksite_service::{
    create_worksite::CreateWorksiteInput, get_worksite::GetWorksiteInput, models::Worksite,
    update_worksite::UpdateWorksiteInput,
};

use crate::{
    components::{
        empty_state::EmptyState,
        page::{PageHeader, PageLayout},
        page_content::PageContent,
        permission_required::PermissionRequired,
        simple_form::{SimpleForm, SimpleFormData},
    },
    routes,
    state::WebHtmxState,
};

pub fn worksite_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::WALLCHART, get(get_wallchart_page))
        .route(routes::WORKSITE, get(get_worksite))
        .route(
            routes::WORKSITE_EDIT_FORM,
            get(get_worksite_edit_form).post(post_worksite_edit_form),
        )
        .route(routes::WORKSITES_CREATE_FORM, get(get_new_worksite_modal))
        .route(routes::WORKSITES, post(post_worksite))
        .with_state(state)
}

struct WorksitePresenter {
    worksite: Worksite, // TODO This should be the out model not domain model
}

impl WorksitePresenter {
    fn new(worksite: Worksite) -> Self {
        Self { worksite }
    }

    pub fn get_worksite_name(&self) -> String {
        self.worksite.name.clone()
    }
}

impl From<WorksitePresenter> for WallchartTableProps {
    fn from(presenter: WorksitePresenter) -> Self {
        let worksite_id = presenter.worksite.id.clone();
        let worksite = presenter.worksite;
        let locations = worksite.locations.clone();

        let locations = locations
            .into_iter()
            .map(|location| {
                let location_id = location.id.clone();

                let add_shift_url = routes::shifts_create_form(&worksite_id, &location_id);

                let shifts = location
                    .shifts
                    .into_iter()
                    .map(|shift| {
                        let shift_id = shift.id.clone();

                        let workers = worksite.get_workers_for_shift(shift_id.clone());

                        let workers = workers
                            .into_iter()
                            .map(|worker| {
                                let tags = worksite.get_tags_for_worker(worker.clone());
                                let worker_id = &worker.id;

                                let assignment_url = routes::shift_assignment(
                                    &worksite_id,
                                    &location_id,
                                    &shift_id,
                                    worker_id,
                                );

                                let details_url = routes::worker(&worksite_id, worker_id);

                                WorkerRowWorker {
                                    full_name: worker.full_name(),
                                    last_assessment: worker
                                        .last_assessment()
                                        .map(|a| a.value.to_string()),
                                    shift_assignment_url: assignment_url,
                                    tags: tags
                                        .into_iter()
                                        .map(|tag| WorkerRowTag {
                                            name: tag.name,
                                            icon: tag.icon,
                                        })
                                        .collect(),
                                    details_url,
                                }
                            })
                            .collect();

                        ShiftRowShift {
                            id: shift.id,
                            name: shift.name,
                            workers,
                        }
                    })
                    .collect();

                LocationRowLocation {
                    id: location.id,
                    name: location.name,
                    add_shift_url,
                    shifts,
                }
            })
            .collect();

        Self {
            worksite_id,
            locations,
        }
    }
}

async fn get_worksite(
    flashes: IncomingFlashes,
    extract::Path(worksite_id): extract::Path<String>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let worksite = worksite_service
        .get_worksite(GetWorksiteInput {
            id: worksite_id.clone(),
        })
        .await
        .unwrap();

    let worksite = match worksite {
        Some(worksite) => worksite,
        None => {
            let empty_state_content = Html(html! {
                <PageLayout header=PageHeader::Title("Not Found".to_string()) >
                    <PageContent>
                        <Card>
                            <EmptyState
                                hx_target=modal_target()
                                hx_swap="beforeend"
                                hx_push_url=routes::worksites_create_form()
                                hx_get=routes::worksites_create_form()
                                button_label="Add new worksite"
                            />
                        </Card>
                    </PageContent>
                </PageLayout>
            });

            return (flashes, empty_state_content);
        }
    };

    let presenter = WorksitePresenter::new(worksite);
    let worksite_name = presenter.get_worksite_name();
    let view_model: WallchartTableProps = presenter.into();

    let html = html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: format!("Wallchart: {}", worksite_name),
                buttons: html! {
                    <SecondaryButton
                        hx_get=routes::locations_create_form(&worksite_id)
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=routes::page_modal_from(routes::locations_create_form(&worksite_id))
                    >
                        Add New Location
                    </SecondaryButton>
                    <SecondaryButton
                        hx_get=routes::worksites_create_form()
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=routes::page_modal_from(routes::worksites_create_form())
                    >
                        Add New Worksite
                    </SecondaryButton>
                    <PrimaryButton
                        hx_get=routes::worksite_edit_form(&worksite_id)
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=routes::page_modal_from(routes::worksite_edit_form(&worksite_id))
                    >
                        Edit Worksite
                    </PrimaryButton>
                }
            }
        >
            <NotificationFlashes flashes=flashes.clone() />
            <PageContent title="Manage your worksite and more">
                    <WallchartTable
                        worksite_id=view_model.worksite_id.clone()
                        locations=view_model.locations.clone()
                    />
            </PageContent>
        </PageLayout>
    };

    (flashes, Html(html))
}

async fn get_wallchart_page() -> impl IntoResponse {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let id = ctx.worksite_id;

    Redirect::temporary(&routes::worksite(&id)).into_response()
}

#[derive(Deserialize, Debug, Clone)]
struct WorksiteFormData {
    worksite_name: String,
}

async fn post_worksite(
    flash: Flash,
    session: Session,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    Form(form): Form<WorksiteFormData>,
) -> impl IntoResponse {
    let worksite = worksite_service
        .create_worksite(CreateWorksiteInput {
            worksite_name: form.worksite_name,
        })
        .await
        .expect("Failed to create worker");

    session.insert_value("selected_worksite_id", worksite.id.clone().into());
    session.insert_value("selected_worksite_name", worksite.name.clone().into());

    (
        StatusCode::OK,
        flash.success("Worksite created successfully!"),
        [
            ("hx-redirect", routes::worksite(&worksite.id)),
            ("hx-retarget", "body".into()),
        ],
    )
}

async fn get_worksite_edit_form(
    extract::Path(worksite_id): extract::Path<String>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let worksite = worksite_service
        .get_worksite(GetWorksiteInput {
            id: worksite_id.clone(),
        })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    Html(html! {
        <Modal>
            <SecondaryHeader
                title="🧑‍🏭 Edit Worksite"
                subtitle="Edit details below."
            />
            <SimpleForm
                action=routes::worksite_edit_form(&worksite_id)
                submit_button_text="Update"
                data=SimpleFormData {
                    name: worksite.name,
                }
            />
        </Modal>
    })
}

#[debug_handler]
async fn post_worksite_edit_form(
    extract::Path(worksite_id): extract::Path<String>,
    flash: Flash,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    Form(form): Form<SimpleFormData>,
) -> impl IntoResponse {
    worksite_service
        .update_worksite(UpdateWorksiteInput {
            worksite_id: worksite_id.clone(),
            worksite_name: form.name,
        })
        .await
        .expect("Failed to create worker");

    (
        StatusCode::OK,
        flash.success("Worksite update successfully!"),
        [
            ("hx-redirect", routes::worksite(&worksite_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[props]
struct WallchartTableProps {
    worksite_id: String,
    locations: Vec<LocationRowLocation>,
}

#[component]
fn WallchartTable(props: WallchartTableProps) -> String {
    html! {
        <div>
            {
                props.locations
                    .into_iter()
                    .map(|location| async {
                        html! {
                            <LocationRow
                                worksite_id=props.worksite_id.clone()
                                location=location
                            />
                        }
                    })
                    .collect_fragment_async()
                    .await
            }
        </div>
    }
}

#[props]
struct LocationRowProps {
    location: LocationRowLocation,

    #[builder(setter(into))]
    worksite_id: String,
}

#[component]
fn LocationRow(props: LocationRowProps) -> String {
    html! {
        <GridLayout>
            <GridCell span=5 class="bg-white py-2 pl-4 px-3 text-left text-xl font-bold text-gray-900">
                {props.location.name}
            </GridCell>
            <GridCell span=1 class="bg-white py-2 pl-4 pr-3 text-right text-sm font-semibold text-gray-900 sm:pl-3">
                <SecondaryButton
                    hx_get=props.location.add_shift_url.clone()
                    hx_push_url=routes::page_modal_from(props.location.add_shift_url.clone())
                    hx_target=modal_target()
                    hx_swap="beforeend"
                >
                    "Add Shift"
                </SecondaryButton>
            </GridCell>
        </GridLayout>
        <ul role="list" class="grid grid-cols-2 gap-6 cols-4 lg:grid-cols-4 m-3">
            {
                props.location
                    .shifts
                    .iter()
                    .map(|shift| async {
                        html! {
                            <li class="col-span-2 flex flex-col divide-y divide-gray-200 rounded-lg bg-white text-center shadow">
                                <ShiftRow
                                    assign_worker_url=routes::shift_assignments_create_form(
                                        &props.worksite_id,
                                        &props.location.id,
                                        &shift.id,
                                    )
                                    shift_name=shift.name.clone()
                                    workers=shift.workers.clone()
                                />
                            </li>
                        }
                    })
                    .collect_fragment_async()
                    .await
            }
        </ul>
    }
}

#[derive(Clone)]
struct ShiftRowShift {
    id: String,
    name: String,
    workers: Vec<WorkerRowWorker>,
}

#[derive(Clone, TypedBuilder)]
struct LocationRowLocation {
    #[builder(setter(into))]
    id: String,

    #[builder(setter(into))]
    name: String,

    #[builder(setter(into))]
    add_shift_url: String,

    shifts: Vec<ShiftRowShift>,
}

#[props]
struct ShiftRowProps {
    #[builder(setter(into))]
    assign_worker_url: String,

    #[builder(setter(into))]
    shift_name: String,

    workers: Vec<WorkerRowWorker>,
}

#[component]
fn ShiftRow(props: ShiftRowProps) -> String {
    html! {
        <table>
            <tr class="border-t border-gray-200">
                <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                    {&props.shift_name}
                </th>
                <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-right text-sm font-semibold text-gray-900 sm:pl-3">
                    <SecondaryButton
                        hx_get=props.assign_worker_url.clone()
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=routes::page_modal_from(props.assign_worker_url.clone())
                    >
                        "Add Worker to Shift"
                    </SecondaryButton>
                </th>
            </tr>
            {
                props
                    .workers
                    .into_iter()
                    .map(|worker| async {
                        html! {
                            <WorkerRow
                                worker=worker
                                shift_name=props.shift_name.clone()
                            />
                        }
                    })
                    .collect_fragment_async()
                    .await
            }
        </table>
    }
}

#[derive(Clone)]
struct WorkerRowTag {
    name: String,
    icon: String,
}

#[derive(Clone, TypedBuilder)]
struct WorkerRowWorker {
    #[builder(setter(into))]
    full_name: String,

    last_assessment: Option<String>,

    #[builder(setter(into))]
    shift_assignment_url: String,

    tags: Vec<WorkerRowTag>,

    #[builder(setter(into))]
    details_url: String,
}

#[props]
struct WorkerRowProps {
    #[builder(setter(into))]
    shift_name: String,

    #[builder(setter(into))]
    worker: WorkerRowWorker,
}

#[component]
fn WorkerRow(props: WorkerRowProps) -> String {
    html! {
        <tr class="border-t border-gray-300" data-loading-states>
            <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">
                  <button
                      hx-get=props.worker.details_url.clone()
                      hx-target=modal_target()
                      hx-swap="beforeend"
                  >
                        {&props.worker.full_name}
                  </button>
            </td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                {props.worker.last_assessment}
            </td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{
                props.worker.tags.into_iter().map(|tag| html! {
                    <span title=tag.name class="cursor-pointer">{tag.icon}</span> }).collect_fragment()
                }
            </td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500 text-right">
                <div class="inline-flex align-right gap-4">
                    <a
                        hx-get=props.worker.details_url.clone()
                        hx-target=modal_target()
                        hx-swap="beforeend"
                        class="cursor-pointer text-indigo-600 hover:text-indigo-900"
                    >
                        Edit<span class="sr-only">, {&props.worker.full_name}</span>
                    </a>
                    <PermissionRequired permission=UserPermission::DeleteAssignedWorker>
                        <a
                            hx-delete=props.worker.shift_assignment_url
                            hx-swap="outerHTML swap:1s"
                            hx-target="closest tr"
                            data-loading-disable
                            hx-confirm="Remove Worker"
                            data-confirm-message=format!("Are you sure you want to remove {} from shift: {}?", &props.worker.full_name, &props.shift_name)
                            class="cursor-pointer text-indigo-600 hover:text-indigo-900"
                        >
                            <div
                                class="htmx-indicator inline-flex animate-spin mr-2 items-center justify-center rounded-full w-4 h-4 bg-gradient-to-tr from-gray-500 to-white"
                            >
                                <span class="inline h-3 w-3 rounded-full bg-white hover:bg-gray-50"></span>
                            </div>

                            Remove<span class="sr-only">, {format!("{}", &props.worker.full_name)}</span>
                        </a>
                    </PermissionRequired>
                </div>
            </td>
        </tr>
    }
}

async fn get_new_worksite_modal() -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <SecondaryHeader
                title="Create Worksite"
            />
            <form hx-post=routes::worksites()>
                <GridLayout>
                    <GridCell>
                        <Label for_input="worksite_name">Name</Label>
                        <TextInput name="worksite_name" />
                    </GridCell>
                    <GridCell>
                        <Button
                            onclick="history.go(-1)"
                            attrs=Attrs::with("data-toggle-action", "close".into())
                        >
                            Cancel
                        </Button>
                        <Button kind="submit">Create</Button>
                    </GridCell>
                </GridLayout>
            </form>
        </Modal>
    })
}
