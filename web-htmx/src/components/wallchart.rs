use rscx::{component, html, props, CollectFragment, CollectFragmentAsync};
use web_client::server::button::SecondaryButton;
use worksite_service::models::{Worker, Worksite};

#[props]
pub struct WallchartProps {
    #[builder(setter(into))]
    worksite: Worksite,
}

#[component]
pub fn Wallchart(props: WallchartProps) -> String {
    let worksite = props.worksite.clone();
    html! {
        <div class="mt-8 flow-root">
            <p><em>Manage your worksite and more.</em></p>
            <div class="mt-4">
                <SecondaryButton
                    hx_get=format!("/wallcharts/{}/locations/new-modal", &props.worksite.id)
                    hx_target="body"
                    hx_swap="beforeend"
                    hx_push_url=format!("/wallcharts/{}/locations/new", &props.worksite.id)
                >
                    "Create New Location"
                </SecondaryButton>
            </div>
            <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
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
                                    location
                                    .shifts
                                    .iter()
                                    .map(|shift| async { html! {
                                        <tr class="border-t border-gray-200">
                                            <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                                                {location.name.clone()} - {shift.name.clone()}
                                            </th>
                                            <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-right text-sm font-semibold text-gray-900 sm:pl-3">
                                                <SecondaryButton
                                                    hx_get=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new-modal", &props.worksite.id, location.clone().id, shift.id)
                                                    hx_target="body"
                                                    hx_swap="beforeend"
                                                    hx_push_url=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", &props.worksite.id, location.clone().id, shift.id)
                                                >
                                                    "Create New Worker"
                                                </SecondaryButton>
                                            </th>
                                        </tr>
                                        <ShiftRows
                                            shift_id=shift.id.clone()
                                            workers=worksite.get_workers_for_shift(shift.id.clone())
                                            location_path=format!("/worksites/{}/locations/{}", &props.worksite.id, location.clone().id)/>
                                    }})
                                    .collect_fragment_async()
                                    .await
                                })
                                .collect_fragment_async()
                                .await
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        </div>

    }
}

#[props]
pub struct ShiftRowsProps {
    #[builder(setter(into))]
    shift_id: String,

    #[builder(setter(into))]
    workers: Vec<Worker>,

    #[builder(setter(into))]
    location_path: String,
}

#[component]
pub fn ShiftRows(props: ShiftRowsProps) -> String {
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
              <a href=format!("/worksites/{}/workers/{}", 1, props.worker.id)>
                  {format!("{} {}", props.worker.first_name, props.worker.last_name)}
              </a>
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
