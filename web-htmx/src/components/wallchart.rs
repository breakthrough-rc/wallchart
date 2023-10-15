use rscx::{component, html, props, CollectFragment, CollectFragmentAsync};

pub struct Worksite {
    pub id: String,
    pub name: String,
    pub locations: Vec<Location>,
}

pub struct Location {
    pub id: String,
    pub name: String,
    pub shifts: Vec<Shift>,
}

pub struct Shift {
    pub id: String,
    pub name: String,
    pub workers: Vec<Worker>,
}

pub struct Worker {
    pub id: String,
    pub name: String,
    pub last_assessment: Assessment,
    pub tags: Vec<Tag>,
}

pub struct Assessment {
    pub id: String,
    pub value: u8,
}

pub struct Tag {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[props]
pub struct WallchartProps {
    #[builder(setter(into))]
    worksite: Worksite,
}

#[component]
pub fn Wallchart(props: WallchartProps) -> String {
    html! {
       <div class="px-4 sm:px-6 lg:px-8">
         <div class="sm:flex sm:items-center">
           <div class="sm:flex-auto">
             <h1 class="text-base font-semibold leading-6 text-gray-900">{props.worksite.name}</h1>
           </div>
         </div>
         <div class="mt-8 flow-root">
           <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
             <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
               <table class="min-w-full">
                 <thead class="bg-white">
                   <tr>
                     <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">Name</th>
                     <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Last Assessment</th>
                     <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Tags</th>
                   </tr>
                 </thead>
                 <tbody class="bg-white">
                   { props
                     .worksite
                     .locations
                     .into_iter()
                     .map(|location| async move { location
                       .shifts
                       .into_iter()
                       .map(|shift| async { html! {
                           <tr class="border-t border-gray-200">
                             <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                               {location.name.clone()} - {shift.name.clone()}
                             </th>
                           </tr>
                           <ShiftRows shift=shift/>
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
       </div>
    }
}

#[props]
pub struct ShiftRowsProps {
    #[builder(setter(into))]
    shift: Shift,
}

#[component]
pub fn ShiftRows(props: ShiftRowsProps) -> String {
    props
        .shift
        .workers
        .into_iter()
        .map(|worker| async {
            html! {
              <WorkerRow worker=worker/>
            }
        })
        .collect_fragment_async()
        .await
}
#[props]
pub struct WorkerRowProps {
    #[builder(setter(into))]
    worker: Worker,
}

#[component]
pub fn WorkerRow(props: WorkerRowProps) -> String {
    html! {
       <tr class="border-t border-gray-300">
         <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">{props.worker.name}</td>
         <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.worker.last_assessment.value}</td>
         <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{props.worker.tags.into_iter().map(|tag| tag.icon).collect_fragment()}</td>
       </tr>
    }
}
