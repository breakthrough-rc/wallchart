#![allow(unused_braces)]
use crate::page::PageLayout;
use auth_service::RequireAuth;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use http::HeaderMap;
use rscx::{component, html, props};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use web_client::server::html_element::HtmlElement;
use web_client::server::modal::Modal;
use web_client::server::notification::{
    NoticationCloseButton, NotificationCall, NotificationPresenter, NotificationTransition,
};
use web_client::server::{
    attrs::Attrs,
    button::{PrimaryButton, SecondaryButton},
};
use web_macros::*;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_playground))
        .route("/test-render", get(get_test_render))
        .route("/htmx", get(htmx_test))
        .route("/ex-business-logic", post(ex_business_logic))
        .route("/custom-notification", get(get_custom_notification))
        .route("/custom-notification2", get(get_custom_notification2))
        .route("/modal/one", get(get_modal_one))
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

#[component]
fn ModalPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">Modal Playground</h2>
            <p><em>Open models and flyouts for fun AND non-profit.</em></p>
            <div class="flex gap-2">
                <PrimaryButton
                    hx_get="/playground/modal/one"
                    hx_target="#modals-root"
                >
                    Open Simple Modal
                </PrimaryButton>
                <PrimaryButton
                    hx_get="/playground/flyout/one"
                    hx_target="#modals-root"
                >
                    Open Flyout
                </PrimaryButton>
            </div>
            <div id="modals-root">
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
                <PrimaryButton
                    onclick="YcControls.showSuccessNotification('Success feels so good!')"
                >
                    Show Success
                </PrimaryButton>
                <PrimaryButton
                    onclick="YcControls.showErrorNotification('This is an error notification.')"
                >
                    Show Error
                </PrimaryButton>
                <PrimaryButton
                    onclick="YcControls.showNotification('This just in', 'You are still not done!')"
                >
                    Show Generic
                </PrimaryButton>
            </div>
            <br />
            <p><em>Show a toast notification (server-side).</em></p>
            <div class="flex gap-2">
                <PrimaryButton
                    hx_post="/playground/ex-business-logic"
                >
                    Show Success
                </PrimaryButton>
                <PrimaryButton
                    hx_get="/playground/custom-notification"
                    hx_target="body"
                    hx_swap="beforeend"
                >
                    Show Custom
                </PrimaryButton>
                <PrimaryButton
                    hx_get="/playground/custom-notification2"
                    hx_target="body"
                    hx_swap="beforeend"
                >
                    Show Custom w/ Standard Components
                </PrimaryButton>
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

async fn get_modal_one() -> Html<String> {
    Html(html! {
        <Modal>
            <h1>I am a very boring and simple modal!</h1>
        </Modal>
    })
}

async fn get_playground() -> Html<String> {
    Html(html! {
        <PageLayout title="Component Playground">
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
                    <SecondaryButton attrs=Attrs::with("data-toggle-action", "close".into())>
                        Close me
                    </SecondaryButton>
                </div>
            </template>
        </NotificationPresenter>
    })
}

async fn get_custom_notification2() -> Html<String> {
    Html(html! {
        <NotificationPresenter call=NotificationCall::Template>
            <template>
                <NotificationTransition
                    class="bg-white border w-full max-w-sm overflow-hidden shadow-lg"
                >
                    <div class="p-4">
                        <div class="flex items-start">
                            <p class="flex-1">Wow this looks a lil nicer.</p>
                            <NoticationCloseButton />
                        </div>
                    </div>
                </NotificationTransition>
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
