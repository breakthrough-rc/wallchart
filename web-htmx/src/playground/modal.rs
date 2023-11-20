#![allow(unused_braces)]
use axum::{response::Html, routing::get, Router};
use rscx::{component, html, props};
use web_client::server::{button::PrimaryButton, flyout::Flyout, modal::Modal};

pub fn modal_routes() -> Router {
    Router::new()
        .route("/modal-one", get(get_modal_one))
        .route("/flyout-one", get(get_flyout_one))
}

// ### Route Handlers ###

async fn get_modal_one() -> Html<String> {
    Html(html! {
        <Modal>
            <h1>I am a very boring and simple modal!</h1>
        </Modal>
    })
}

async fn get_flyout_one() -> Html<String> {
    Html(html! {
        <Flyout title="Hello Playground!">
            <h1>Sliding on in... its a flyout!</h1>
        </Flyout>
    })
}

// ### Components ###

#[component]
pub fn ModalPlayground() -> String {
    html! {
        <section class="py-8">
            <h2 class="text-xl font-bold">Modal Playground</h2>
            <p><em>Open models and flyouts for fun AND non-profit.</em></p>
            <div class="flex gap-2">
                <PrimaryButton
                    hx_get="/playground/modals/modal-one"
                    hx_target="#modals-root"
                >
                    Open Simple Modal
                </PrimaryButton>
                <PrimaryButton
                    hx_get="/playground/modals/flyout-one"
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
