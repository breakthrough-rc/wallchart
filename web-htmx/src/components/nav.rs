use rscx::{component, html, props, CollectFragment};

use web_client::server::modal::modal_target;
use web_client::server::popup_menu::{Menu, PopupMenu};

use crate::components::logo::Logo;
use crate::routes;

fn profile_links() -> Vec<(String, String)> {
    vec![
        ("Your Profile".into(), "#".into()),
        ("Settings".into(), "#".into()),
        ("Sign out".into(), "#".into()),
    ]
}

#[component]
pub fn Nav() -> String {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");

    let worksite_id = ctx.worksite_id.clone();
    let worksite_name = ctx.worksite_name.clone();

    let nav_links = [
        ("Wallchart", routes::wallchart()),
        ("Workers", routes::workers(&worksite_id)),
        ("Tags", routes::tags(&worksite_id)),
        ("Report", "/report".into()),
        ("Users", routes::users()),
        ("Import", routes::csv_upload()),
    ];

    html! {
        <nav class="border-b border-gray-200 bg-white">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                <div class="flex h-16 justify-between">
                    <div class="flex">
                        <div class="flex flex-shrink-0 items-center">
                            // <img class="block h-8 w-auto lg:hidden" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company" />
                            <div class="h-8 w-8">
                                <a href="/"><Logo /></a>
                            </div>
                            // <img class="hidden h-8 w-auto lg:block" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company" />
                        </div>
                        <div class="hidden sm:-my-px sm:ml-6 sm:flex sm:space-x-8">
                            {
                                nav_links
                                    .clone()
                                    .into_iter()
                                    .map(|(label, href)| {
                                        let is_current = ctx.page_url == href;
                                        let link_css = "inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";
                                        let link_css = if is_current {
                                            format!("border-indigo-500 text-gray-900 {}", link_css)
                                        } else {
                                            format!("border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 {}", link_css)
                                        };

                                        html! {
                                            <a
                                                href=href
                                                class=link_css
                                                aria-current=if is_current { "page" } else { "" }
                                            >
                                                {label}
                                            </a>
                                        }
                                    })
                                    .collect_fragment()
                            }
                        </div>
                    </div>
                    <div class="hidden sm:ml-6 sm:flex sm:items-center">
                        <a
                            hx-get=routes::selected_worksite_modal()
                            hx-target=modal_target()
                            hx-swap="beforeend"
                            class="cursor-pointer text-indigo-600 hover:text-indigo-900 whitespace-pre-line"
                        >
                            {worksite_name.clone()} " ⌄"
                        </a>
                        <ProfileDropdown />
                    </div>

                    <div class="-mr-2 flex items-center sm:hidden">
                        // Mobile menu button
                        <button type="button" class="relative inline-flex items-center justify-center rounded-md bg-white p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" aria-controls="mobile-menu" aria-expanded="false">
                            <span class="absolute -inset-0.5"></span>
                            <span class="sr-only">Open main menu</span>

                            // Menu open: "hidden", Menu closed: "block"
                            <svg class="block h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                            </svg>

                            // Menu open: "block", Menu closed: "hidden"
                            <svg class="hidden h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>

            // Mobile menu, show/hide based on menu state.
            <div class="sm:hidden" id="mobile-menu">
                <div class="space-y-1 pb-3 pt-2">
                    {
                        nav_links
                            .into_iter()
                            .map(|(label, href)| {
                                let is_current = false; // TODO! Fix me!
                                let link_css = "block border-l-4 py-2 pl-3 pr-4 text-base font-medium";
                                let link_css = if is_current {
                                    format!("border-indigo-500 bg-indigo-50 text-indigo-700 {}", link_css)
                                } else {
                                    format!("border-transparent text-gray-600 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-800 {}", link_css)
                                };

                                html! {
                                    <a
                                        href=href
                                        class=link_css
                                        aria-current=if is_current { "page" } else { "" }
                                    >
                                        {label}
                                    </a>
                                }
                            })
                            .collect_fragment()
                    }
                </div>

                <div class="border-t border-gray-200 pb-3 pt-4">
                    <div class="flex items-center px-4">
                        <div class="flex-shrink-0">
                            <img class="h-10 w-10 rounded-full" src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" />
                        </div>
                        <div class="ml-3">
                            <div class="text-base font-medium text-gray-800">Tom Cook</div>
                            <div class="text-sm font-medium text-gray-500">tom@example.com</div>
                        </div>
                        <button type="button" class="relative ml-auto flex-shrink-0 rounded-full bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                            <span class="absolute -inset-1.5"></span>
                            <span class="sr-only">View notifications</span>
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                            </svg>
                        </button>
                    </div>
                    <div class="mt-3 space-y-1">
                        {
                            profile_links()
                                .into_iter()
                                .map(|(label, href)| html! {
                                    <a href=href class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800">{label}</a>
                                })
                                .collect_fragment()
                        }
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn ProfileDropdown() -> String {
    html! {
        <PopupMenu
            id="user-nav-popupmenu"
            class="ml-3"
            button_class="flex max-w-xs items-center rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            button_content=html! {
                <img
                    class="h-8 w-8 rounded-full"
                    src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                    alt="user-avatar"
                />
            }
        >
            <Menu
                id="user-nav-menu"
                links=profile_links().into_iter().map(|link| link.into()).collect()
            />
        </PopupMenu>
    }
}
