#![allow(unused_braces)]
use crate::page::PageLayout;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use http::HeaderMap;
use rscx::{component, html, props};
use std::time::{SystemTime, UNIX_EPOCH};
use web_client::server::notification::{NotificationCall, NotificationPresenter};
use web_client::{html_attrs, server::html_element::HtmlElement};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_playground))
        .route("/test-render", get(get_test_render))
        .route("/htmx", get(htmx_test))
        .route("/ex-business-logic", post(ex_business_logic))
        .route("/custom-notification", get(get_custom_notification))
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
html_attrs! {
    pub struct SimpleElementProps {
        #[builder(default)]
        children: String,

        #[builder(default="SIMPLE!".to_string())]
        simple: String,
    }
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
                <SimpleElement class="font-bold".into() simple="YO".into()>
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

#[component]
fn NotificationsPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">Notifications Playground</h2>
            <p><em>Show a toast notification (client-side).</em></p>
            <div class="flex gap-2">
                <button
                    class="bg-slate-200 p-3 rounded-full"
                    onclick="YcControls.showSuccessNotification('Success feels so good!')"
                >
                    Show Success
                </button>
                <button
                    class="bg-slate-200 p-3 rounded-full"
                    onclick="YcControls.showErrorNotification('This is an error notification.')"
                >
                    Show Error
                </button>
                <button
                    class="bg-slate-200 p-3 rounded-full"
                    onclick="YcControls.showNotification('This just in', 'You are still not done!')"
                >
                    Show Generic
                </button>
            </div>
            <br />
            <p><em>Show a toast notification (server-side).</em></p>
            <div class="flex gap-2">
                <button
                    class="bg-slate-200 p-3 rounded-full"
                    hx-post="/playground/ex-business-logic"
                >
                    Show Success
                </button>
                <button
                    class="bg-slate-200 p-3 rounded-full"
                    hx-get="/playground/custom-notification"
                    hx-target="body"
                    hx-swap="beforeend"
                >
                    Show Custom
                </button>
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
                "I didn't think so!"
            </marquee>
            <section class="py-8">
                <h2 class="text-xl font-bold">HTMX Rendering</h2>
                <div class="flex gap-2">
                    <button
                        class="bg-slate-200 p-3 rounded-full"
                        hx-get="/playground/htmx"
                        hx-swap="outerHTML"
                    >
                        Click me!
                    </button>
                </div>
            </section>
            <NotificationsPlayground />
            <HtmlElementPlayground />
            <PartialRenderTest />
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

async fn get_custom_notification() -> Html<String> {
    Html(html! {
        <NotificationPresenter call=NotificationCall::Template>
            <template>
                <div class="bg-white p-10 border">
                    <p>This is a bad notification!</p>
                    <button data-toggle-action="close">Close me</button>
                </div>
            </template>
        </NotificationPresenter>
    })
}

async fn ex_business_logic() -> Html<String> {
    // Do some business logic here...
    // Then we will tell the client to show a success notification.
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    Html(html! {
        {format!("Action complete. Result: {}!", nanos)}
        <NotificationPresenter call=NotificationCall::Success(format!("Server side validated! Answer is {}", nanos)) />
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
