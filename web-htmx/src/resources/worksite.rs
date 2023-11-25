use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_flash::IncomingFlashes;
use rscx::{component, html, props, CollectFragment, CollectFragmentAsync};

use web_client::server::{
    button::{PrimaryButton, SecondaryButton},
    card::Card,
    modal::modal_target,
    notification::NotificationFlashes,
};
use worksite_service::{
    get_worksite::GetWorksiteInput,
    models::{Location as LocationModel, Tag, Worker, Worksite},
};

use crate::{
    components::{
        page::{PageHeader, PageLayout},
        page_content::PageContent,
    },
    routes::{self, locations_new, locations_new_modal, tags, WALLCHART},
    state::WebHtmxState,
};

pub fn worksite_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(WALLCHART, get(get_wallchart_page))
        .with_state(state)
}

async fn get_wallchart_page(
    flashes: IncomingFlashes,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let id: &str = "1";

    let worksite = worksite_service
        .get_worksite(GetWorksiteInput { id: id.to_string() })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    let worksite_name = worksite.name.clone();

    let html = html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: format!("Wallchart: {}", worksite_name),
                buttons: html! {
                    <SecondaryButton
                        hx_get=locations_new_modal(&id.into())
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=locations_new(&id.into())
                    >
                        Add New Location
                    </SecondaryButton>
                    <SecondaryButton
                        tag="a"
                        href=tags(&id.into())
                    >
                        Manage Tags
                    </SecondaryButton>
                    <PrimaryButton
                        onclick="alert('Coming soon!')"
                    >
                        Edit Worksite
                    </PrimaryButton>
                }
            }
        >
            <NotificationFlashes flashes=flashes.clone() />
            <p><em>Manage your worksite and more.</em></p>
            <PageContent>
                <Card>
                    <WallchartTable worksite=worksite/>
                </Card>
            </PageContent>
        </PageLayout>
    };

    (flashes, Html(html))
}

#[props]
pub struct WallchartTableProps {
    #[builder(setter(into))]
    worksite: Worksite,
}

#[component]
pub fn WallchartTable(props: WallchartTableProps) -> String {
    let worksite = props.worksite.clone();
    html! {
        <table class="min-w-full">
            <thead class="bg-white">
                <tr>
                    <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">Name</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Last Assessment</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Tags</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"></th>
                </tr>
            </thead>
            <tbody class="bg-white">
                {
                    &worksite.locations
                        .iter()
                        .map(|location| async {
                            html! {
                                <LocationRow
                                    location=location.clone()
                                    worksite=worksite.clone()
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
pub struct LocationRowProps {
    #[builder(setter(into))]
    location: LocationModel,

    #[builder(setter(into))]
    worksite: Worksite,
}

#[component]
pub fn LocationRow(props: LocationRowProps) -> String {
    html! {
        <tr class="border-t border-gray-200">
            <th colspan="3" scope="colgroup" class="bg-gray-200 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                {props.location.name}
            </th>
            <th colspan="3" scope="colgroup" class="bg-gray-200 py-2 pl-4 pr-3 text-right text-sm font-semibold text-gray-900 sm:pl-3">
                <SecondaryButton
                    hx_get=routes::shifts_new_modal(&props.worksite.id, &props.location.id)
                    hx_push_url=routes::shifts_new(&props.worksite.id, &props.location.id)
                    hx_target=modal_target()
                    hx_swap="beforeend"
                >
                    "Add Shift"
                </SecondaryButton>
            </th>
        </tr>
        {
            props.location
                .shifts
                .iter()
                .map(|shift| async {
                    html! {
                        <ShiftRow
                            shift_id=shift.id.clone()
                            shift_name=shift.name.clone()
                            workers=props.worksite.get_workers_for_shift(shift.id.clone())
                            worksite=props.worksite.clone()
                            new_worker_action=routes::shift_assignments_new_modal(&props.worksite.id, &props.location.id, &shift.id)
                            new_worker_push_url=routes::shift_assignments_new(&props.worksite.id, &props.location.id, &shift.id)
                            location_id=props.location.id.clone()
                        />
                    }
                })
                .collect_fragment_async()
                .await
        }
    }
}

#[props]
pub struct ShiftRowProps {
    #[builder(setter(into))]
    shift_id: String,

    #[builder(setter(into))]
    shift_name: String,

    workers: Vec<Worker>,
    worksite: Worksite,

    #[builder(setter(into))]
    location_id: String,

    #[builder(setter(into))]
    new_worker_action: String,

    #[builder(setter(into))]
    new_worker_push_url: String,
}

#[component]
pub fn ShiftRow(props: ShiftRowProps) -> String {
    html! {
        <tr class="border-t border-gray-200">
            <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                {props.shift_name}
            </th>
            <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-right text-sm font-semibold text-gray-900 sm:pl-3">
                <SecondaryButton
                    hx_get=props.new_worker_action
                    hx_target=modal_target()
                    hx_swap="beforeend"
                    hx_push_url=props.new_worker_push_url
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
                            tags=props.worksite.get_tags_for_worker(worker.clone())
                            worker_action=routes::worker(&props.worksite.id, &worker.clone().id)
                            shift_assignment_action=routes::shift_assignment(&props.worksite.id, &props.location_id, &props.shift_id, &worker.clone().id)
                            worker=worker
                        />
                    }
                })
                .collect_fragment_async()
                .await
        }
    }
}

#[props]
pub struct WorkerRowProps {
    worker: Worker,
    tags: Vec<Tag>,

    #[builder(setter(into))]
    worker_action: String,

    #[builder(setter(into))]
    shift_assignment_action: String,
}

#[component]
pub fn WorkerRow(props: WorkerRowProps) -> String {
    html! {
        <tr class="border-t border-gray-300" data-loading-states>
            <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">
                  <button
                      hx-get=props.worker_action
                      hx-target=modal_target()
                      hx-swap="beforeend"
                  >
                      {format!("{} {}", props.worker.first_name, props.worker.last_name)}
                  </button>
            </td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.worker.last_assessment().map(|assessment| assessment.value).unwrap_or(0)}</td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.tags.into_iter().map(|tag| tag.icon).collect_fragment()}</td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                <button
                    type="button"
                    hx-delete={props.shift_assignment_action}
                    class="text-center inline-flex items-center rounded bg-white px-2 py-1 text-xs font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 disabled:bg-gray-50 disabled:shadow-none disabled:cursor-not-allowed disabled:text-gray-500"
                    hx-swap="outerHTML swap:1s"
                    hx-target="closest tr"
                    data-loading-disable
                >
                    <div
                        class="htmx-indicator inline-flex animate-spin mr-2 items-center justify-center rounded-full w-4 h-4 bg-gradient-to-tr from-gray-500 to-white"
                    >
                        <span class="inline h-3 w-3 rounded-full bg-white hover:bg-gray-50"></span>
                    </div>
                    Remove
                </button>
            </td>
        </tr>
    }
}
