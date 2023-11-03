use super::transition::Transition;
use rscx::{component, html, props};

#[component]
pub(crate) fn NotificationLiveRegion() -> String {
    html! {
        <div id="notification-live-region" aria-live="assertive" class="pointer-events-none fixed inset-0 flex items-end px-4 py-6 sm:items-start sm:p-6">
            <section class="flex w-full flex-col items-center space-y-4 sm:items-end">
            </section>

            <template id="tpl-error-notification">
                <ErrorNotification />
            </template>
        </div>
    }
}

#[component]
pub fn ErrorNotification() -> String {
    html! {
        <Transition
            class="pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg bg-white shadow-lg ring-1 ring-black ring-opacity-5"
            enter="transform ease-out duration-300 transition"
            enter_from="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2"
            enter_to="translate-y-0 opacity-100 sm:translate-x-0"
            leave="transition ease-in duration-300"
            leave_from="opacity-100"
            leave_to="opacity-0"
        >
            <div class="p-4">
                <div class="flex items-start">
                <div class="flex-shrink-0">
                    <svg class="h-6 w-6 text-red-400" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </div>
                <div class="ml-3 w-0 flex-1 pt-0.5">
                    <p class="text-sm font-medium text-gray-900">Oops! Something went wrong.</p>
                    <p class="mt-1 text-sm text-gray-500" data-error-message></p>
                </div>
                <div class="ml-4 flex flex-shrink-0">
                    <button type="button" data-toggle-action="close" data-notification-close class="inline-flex rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                        <span class="sr-only">Close</span>
                        <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" data-notification-close>
                            <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
                        </svg>
                    </button>
                </div>
                </div>
            </div>
        </Transition>
    }
}
