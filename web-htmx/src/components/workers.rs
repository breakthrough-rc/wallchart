use rscx::{component, html, props, CollectFragment, CollectFragmentAsync};
use worksite_service::models::{Tag, Worker, Worksite};

#[props]
pub struct WorkersProps {
    worksite: Worksite,
    workers: Vec<Worker>,
}

#[component]
pub fn Workers(props: WorkersProps) -> String {
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
                                <Worker
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
pub struct WorkerProps {
    worker: Worker,
    tags: Vec<Tag>,
}

#[component]
pub fn Worker(props: WorkerProps) -> String {
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
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.tags.into_iter().map(|tag| tag.icon).collect_fragment()}</td>
        </tr>
    }
}
