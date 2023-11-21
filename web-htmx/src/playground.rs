#![allow(unused_braces)]
use crate::components::page::PageLayout;
use axum::{response::Html, routing::get, Router};
use http::HeaderMap;
use rscx::{component, html, props};

use modal::{modal_routes, ModalPlayground};
use notifications::{notification_routes, NotificationsPlayground};
use web_client::server::{
    button::{PrimaryButton, SecondaryButton},
    html_element::HtmlElement,
};
use web_macros::*;

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

#[component]
fn FooButton() -> String {
    html! {
        <HtmlElement
            tag="button"
            class="bg-slate-200 ml-4 p-3 rounded-full"
            id="btn-foo"
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
            tag="button"
            id="btn-alert"
            class="bg-slate-200 ml-4 p-3 rounded-full"
            onclick={format!("alert('{}')", props.message)}
        >
            {props.children}
        </HtmlElement>
    }
}

// This macro adds all standard HTML attributes for your component!
#[html_element]
pub struct SimpleElementProps {
    #[builder(default)]
    children: String,

    #[builder(default="SIMPLE!".to_string())]
    simple: String,

    #[builder(setter(into), default=String::from("div"))]
    tag: String,
}

#[component]
fn SimpleElement(props: SimpleElementProps) -> String {
    html! {
        <div class=props.class data-simple=props.simple data-tag=props.tag>
            <p>I am foo, hear me roar!</p>
            <div>{props.children}</div>
        </div>
    }
}

#[component]
fn HtmlElementPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">"HtmlElement Playground"</h2>
            <div class="flex flex-col gap-4">
                <SimpleElement class="font-bold" simple="YO".into()>
                    Simple but not so simple.
                </SimpleElement>
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
            <section class="py-8">
                <h2 class="text-xl font-bold">Auth Testing</h2>
                <div class="flex gap-2">
                    <PrimaryButton
                        tag="a"
                        href="/wallchart"
                    >
                        Authenticated page link
                    </PrimaryButton>
                </div>
            </section>
        </Welcome>
    }
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
