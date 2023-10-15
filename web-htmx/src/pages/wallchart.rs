use axum::response::Html;
use rscx::html;

use crate::page::PageLayout;

pub async fn get_wallchart_page() -> Html<String> {
    Html(html! {
    <PageLayout>
      <div class="px-4 sm:px-6 lg:px-8">
        <div class="sm:flex sm:items-center">
          <div class="sm:flex-auto">
            <h1 class="text-base font-semibold leading-6 text-gray-900">Dunder Mifflin</h1>
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
                  <tr class="border-t border-gray-200">
                    <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                      Office - Day
                    </th>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Jim Halpert</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">1</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🔴 🔵 🟢 🟡"</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Pam Beesly</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">2</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🟢 🟡"</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Dwight Schrute</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">4</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">""</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Creed Bratton</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">3</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🟢 🟡"</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Angela Martin</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">5</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🟢"</td>
                  </tr>

                  <tr class="border-t border-gray-200">
                    <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                      Warehouse - Day
                    </th>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Darryl Philbin</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">1</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🔵 🟢 🟡"</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Nate Nickerson</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">3</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🟡"</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Roy Anderson</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">3</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">""</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Madge Madsen</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">4</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">""</td>
                  </tr>

                  <tr class="border-t border-gray-200">
                    <th colspan="3" scope="colgroup" class="bg-gray-50 py-2 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">
                      Warehouse - Night
                    </th>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Val Johnson</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">2</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🔵 🟢 🟡"</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Lonny Collins</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">1</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">"🟡"</td>
                  </tr>
                  <tr class="border-t border-gray-300">
                    <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">Frank</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">3</td>
                    <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">""</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </PageLayout>
    })
}
