use axum::{http::HeaderMap, response::Html, routing::get, Router};
use page::PageLayout;
use pages::{wallchart::get_wallchart_page, workers::get_workers_new_page};
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
        .route("/workers/new", get(get_workers_new_page))
        .route("/test-render", get(get_test_render))
        .route("/htmx", get(htmx_test))
        .nest_service("/client", client_routes())
        .with_state(state)
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
