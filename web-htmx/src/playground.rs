#![allow(unused_braces)]
use crate::page::PageLayout;
use axum::{response::Html, routing::get, Router};
use http::HeaderMap;
use rscx::{component, html, props};
use web_client::server::html_element::HtmlElement;

pub fn routes() -> Router {
    Router::new()
        .route("/playground", get(get_playground))
        .route("/test-render", get(get_test_render))
        .route("/htmx", get(htmx_test))
}

#[component]
fn FooButton() -> String {
    html! {
        <HtmlElement
            tag="button".into()
            class="bg-slate-200 ml-4 p-3 rounded-full".into()
            id="btn-foo".into()
        >
            A button rendered w/ HTMLElement. Click for more foo!
        </HtmlElement>
    }
}

#[props]
struct MessageButtonProps {
    #[builder(default)]
    message: String,

    #[builder(default)]
    children: String,
}

#[component]
fn MessageButton(props: MessageButtonProps) -> String {
    html! {
        <HtmlElement
            tag="button".into()
            id="btn-alert".into()
            class="bg-slate-200 ml-4 p-3 rounded-full".into()
            onclick={format!("alert('{}')", props.message).into()}
        >
            {props.children}
        </HtmlElement>
    }
}

#[component]
fn HtmlElementPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">"HtmlElement Playground"</h2>
            <div class="flex flex-col gap-4">
                <FooButton />
                <MessageButton message="This is a message from a button!".into()>
                    I am a MessageButton, click to see a message!
                </MessageButton>
            </div>
        </section>
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
        </section>
    }
}

#[component]
pub fn PlaygroundPgContent() -> String {
    html! {
        <Welcome
            title="Yall Ready for This?"
        >
            <marquee>
                "I didn't think so!"
            </marquee>
            <section class="py-8">
                <h2 class="text-xl font-bold">Click some buttons</h2>
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
            </section>
            <HtmlElementPlayground />
        </Welcome>
    }
}

// ### Route Handlers ###

async fn get_playground() -> Html<String> {
    Html(html! {
        <PageLayout>
            <PlaygroundPgContent />
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
