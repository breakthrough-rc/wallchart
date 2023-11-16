use rscx::{component, html, props, CollectFragment, CollectFragmentAsync};
use web_client::server::button::SecondaryButton;
use worksite_service::models::{Location as LocationModel, Worker, Worksite};

#[props]
pub struct WallchartProps {
    #[builder(setter(into))]
    worksite: Worksite,
}

#[component]
pub fn Wallchart(props: WallchartProps) -> String {
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
                    &worksite
                    .locations
                    .iter()
                    .map(|location| async {
                        html! {
                            <Location
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
pub struct LocationProps {
    #[builder(setter(into))]
    location: LocationModel,

    #[builder(setter(into))]
    worksite: Worksite,
}

#[component]
pub fn Location(props: LocationProps) -> String {
    html! {
        <tr class="border-t border-gray-200">
            <th colspan="3" scope="colgroup" class="bg-gray-200 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                {props.location.name}
            </th>
            <th colspan="3" scope="colgroup" class="bg-gray-200 py-2 pl-4 pr-3 text-right text-sm font-semibold text-gray-900 sm:pl-3">
                <SecondaryButton
                    hx_get=format!("/wallcharts/{}/locations/{}/shifts/new-modal", &props.worksite.id, &props.location.id)
                    hx_push_url=format!("/wallcharts/{}/locations/{}/shifts/new", &props.worksite.id, &props.location.id)
                    hx_target="body"
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
                        <Shift
                            shift_id=shift.id.clone()
                            shift_name=shift.name.clone()
                            workers=props.worksite.get_workers_for_shift(shift.id.clone())
                            new_worker_action=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new-modal", &props.worksite.id, props.location.id, shift.id)
                            new_worker_push_url=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", &props.worksite.id, props.location.id, shift.id)
                            location_path=format!("/worksites/{}/locations/{}", &props.worksite.id, props.location.id)
                        />
                    }
                })
                .collect_fragment_async()
                .await
        }
    }
}

#[props]
pub struct ShiftProps {
    #[builder(setter(into))]
    shift_id: String,

    #[builder(setter(into))]
    shift_name: String,

    #[builder(setter(into))]
    workers: Vec<Worker>,

    #[builder(setter(into))]
    location_path: String,

    #[builder(setter(into))]
    new_worker_action: String,

    #[builder(setter(into))]
    new_worker_push_url: String,
}

#[component]
pub fn Shift(props: ShiftProps) -> String {
    html! {
        <tr class="border-t border-gray-200">
            <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                {props.shift_name}
            </th>
            <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-right text-sm font-semibold text-gray-900 sm:pl-3">
                <SecondaryButton
                    hx_get=props.new_worker_action
                    hx_target="body"
                    hx_swap="beforeend"
                    hx_push_url=props.new_worker_push_url
                >
                    "Add Worker"
                </SecondaryButton>
            </th>
        </tr>
        {
            props
                .workers
                .into_iter()
                .map(|worker| async {
                    html! {
                      <WorkerRow worker=worker shift_path=format!("{}/shifts/{}", props.location_path, props.shift_id)/>
                    }
                })
                .collect_fragment_async()
                .await
        }
    }
}

#[props]
pub struct WorkerRowProps {
    #[builder(setter(into))]
    worker: Worker,

    #[builder(setter(into))]
    shift_path: String,
}

#[component]
pub fn WorkerRow(props: WorkerRowProps) -> String {
    html! {
      <tr class="border-t border-gray-300" data-loading-states>
          <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">
                <button
                    hx-get=format!("/worksites/{}/workers/{}", 1, props.worker.id)
                    hx-target="body"
                    hx-swap="beforeend"
                >
                    {format!("{} {}", props.worker.first_name, props.worker.last_name)}
                </button>
          </td>
          <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.worker.last_assessment.map(|assessment| assessment.value).unwrap_or(0)}</td>
          <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.worker.tags.into_iter().map(|tag| tag.icon).collect_fragment()}</td>
          <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
              <button
                  type="button"
                  hx-delete={format!("{}/workers/{}", props.shift_path, props.worker.id)}
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
