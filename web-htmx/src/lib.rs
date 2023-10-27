use axum::{
    http::HeaderMap,
    response::Html,
    routing::{delete, get},
    Router,
};
use http::StatusCode;
use page::PageLayout;
use pages::{
    wallchart::get_wallchart_page,
    workers::{get_workers_new_page, post_workers_new_page},
};
//##PLOP USE RESOURCE HOOK##
use resources::worksite::delete_worker_from_shift;
use rscx::{component, html, props};
use state::WebHtmxState;
use web_client::routes as client_routes;

pub mod components;
pub mod livereload;
pub mod page;
pub mod pages;
pub mod resources;
pub mod state;

pub fn routes(state: WebHtmxState) -> Router {
    Router::new()
        .route("/", get(get_home))
        .route("/wallchart", get(get_wallchart_page))
        .route(
            "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/:worker_id",
            delete(delete_worker_from_shift),
        )
        .route(
            "/wallcharts/:wallchart_id/locations/:location_id/shifts/:shift_id/workers/new",
            get(get_workers_new_page).post(post_workers_new_page),
        )
        .route("/test-render", get(get_test_render))
        .route("/htmx", get(htmx_test))
        .nest_service("/client", client_routes())
        .fallback(fallback)
        .with_state(state.clone())
    //##PLOP MERGE ROUTE HOOK##
}

async fn fallback() -> (StatusCode, Html<String>) {
    let not_found = html! {
        <PageLayout title="Oops!".into()>
            <main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
                <div class="text-center">
                    <p class="text-base font-semibold text-indigo-600">404</p>
                    <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">Page not found</h1>
                    <p class="mt-6 text-base leading-7 text-gray-600">"Sorry, we couldn’t find the page you’re looking for."</p>
                    <div class="mt-10 flex items-center justify-center gap-x-6">
                        <a href="/" class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Go back home</a>
                        <a href="/support" class="text-sm font-semibold text-gray-900">Contact support <span aria-hidden="true">"&rarr;"</span></a>
                    </div>
                </div>
            </main>
        </PageLayout>
    };

    (StatusCode::NOT_FOUND, Html(not_found))
}

#[props]
struct WelcomeProps {
    #[builder(setter(into), default = "Welcome!".to_string())]
    title: String,

    #[builder(default)]
    children: String,
}

#[component]
fn Welcome(props: WelcomeProps) -> String {
    html! {
        <h1 class="text-xl text-slate-600">{props.title}</h1>
        {props.children}
    }
}

async fn get_home() -> Html<String> {
    Html(html! {
        <PageLayout>
            <Welcome
                title="Yall Ready for This?"
            >
                <marquee>
                    "I didn't think so!"
                </marquee>
                <button
                    class="bg-slate-200 p-3 rounded-full"
                    hx-get="/htmx"
                    hx-swap="outerHTML"
                >
                    Click me!
                </button>
                <button
                    class="bg-slate-200 ml-4 p-3 rounded-full"
                    onclick="YcControls.showErrorNotification('This is an error notification.')"
                >
                    Show Error Notification
                </button>
                <div>
                    <h2>"Test rendering"</h2>
                    <ul class="list-disc list-inside">
                        <li>
                            <a
                                class="text-blue-600 hover:underline"
                                href="/test-render"
                            >
                                "Goto a full page render."
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-blue-600 hover:underline"
                                hx-get="/test-render"
                                hx-target=".partial-rendered-content"
                            >
                                "See a partial render."
                            </a>
                        </li>
                    </ul>
                    <div class="text-sm italic partial-rendered-content"></div>

                </div>
            </Welcome>
        </PageLayout>
    })
}

async fn htmx_test() -> Html<String> {
    Html("Is this the real life? Is this just fantasy?".into())
}

// Test to see if we can partial render a component that includes PageLayout.
async fn get_test_render(headers: HeaderMap) -> Html<String> {
    Html(html! {
        <PageLayout
            partial=headers.contains_key("Hx-Request")
        >
            <section>
                <h1>"Test Render"</h1>
                <p>
                    "If you are viewing this page at the url `/test-render`
                    you should see the full render (header and footer)."
                </p>
                <p>
                    "If this is being pulled in from an htmx request
                    we should just see the `section` tag only."
                </p>
            </section>
        </PageLayout>
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
