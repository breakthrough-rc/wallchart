#![allow(unused_braces)]
use crate::components::page::PageLayout;
use axum::{response::Html, routing::get, Router};
use http::HeaderMap;
use rscx::{component, html, props};

use auth::AuthPlayground;
use html_element::HtmlElementPlayground;
use modal::{modal_routes, ModalPlayground};
use notifications::{notification_routes, NotificationsPlayground};
use web_client::server::button::SecondaryButton;

pub mod auth;
pub mod html_element;
pub mod modal;
pub mod notifications;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_playground))
        .route("/test-render", get(get_test_render))
        .route("/htmx", get(htmx_test))
        .nest("/modals", modal_routes())
        .nest("/notifications", notification_routes())
}

// ### Route Handlers ###

async fn get_playground() -> Html<String> {
    Html(html! {
        <PageLayout header="Component Playground">
            <PlaygroundPgContent />
        </PageLayout>
    })
}

async fn htmx_test() -> Html<String> {
    Html("Is this the real life? Is this just fantasy?".into())
}

// ### Components ###

#[component]
pub fn PlaygroundPgContent() -> String {
    html! {
        <Welcome
            title="Yall Ready for This?"
        >
            <marquee>
                "It's The Playground&#133; Let's have some fun!"
            </marquee>
            <section class="py-8">
                <h2 class="text-xl font-bold">HTMX Rendering</h2>
                <div class="flex gap-2">
                    <SecondaryButton
                        hx_get="/playground/htmx"
                        hx_swap="outerHTML"
                    >
                        Click me!
                    </SecondaryButton>
                </div>
            </section>
            <NotificationsPlayground />
            <ModalPlayground />
            <HtmlElementPlayground />
            <PartialRenderTest />
            <AuthPlayground />
        </Welcome>
    }
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

#[component]
fn PartialRenderTest() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">"Test rendering"</h2>
            <ul class="list-disc list-inside">
                <li>
                    <a
                        class="text-blue-600 hover:underline"
                        href="/playground/test-render"
                    >
                        "Goto a full page render."
                    </a>
                </li>
                <li>
                    <a
                        class="text-blue-600 hover:underline"
                        hx-get="/playground/test-render"
                        hx-target=".partial-rendered-content"
                    >
                        "See a partial render."
                    </a>
                </li>
            </ul>
            <div class="text-sm italic partial-rendered-content"></div>
        </section>
    }
}

// Test to see if we can partial render a component that includes PageLayout.
async fn get_test_render(headers: HeaderMap) -> Html<String> {
    Html(html! {
        <PageLayout
            partial=headers.contains_key("Hx-Request")
        >
            <section>
                <h1>Test Render</h1>
                <p>
                    "If you are viewing this page at the url `test-render`
                    you should see the full render (header and footer)"
                </p>
                <p>
                    "If this is being pulled in from an htmx request
                    we should just see the `section` tag only."
                </p>
            </section>
        </PageLayout>
    })
}
