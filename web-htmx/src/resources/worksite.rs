use crate::components::{
    page::{PageHeader, PageLayout},
    wallchart::Wallchart,
};
use crate::state::WebHtmxState;
use axum::{
    extract::{State},
    response::{Html, IntoResponse},
    routing::{get},
    Router,
};
use axum_flash::IncomingFlashes;

use rscx::html;

use web_client::server::{
    button::{PrimaryButton, SecondaryButton},
    notification::NotificationFlashes,
};
use worksite_service::{
    get_worksite::GetWorksiteInput,
};

pub fn worksite_routes(state: WebHtmxState) -> Router {
    Router::new()
        // The actual wallchart
        .route("/wallchart", get(get_wallchart_page))
        .with_state(state)
}

async fn get_wallchart_page(
    flashes: IncomingFlashes,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let id: &str = "1";

    let worksite = worksite_service
        .get_worksite(GetWorksiteInput { id: id.to_string() })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    let worksite_name = worksite.name.clone();

    let html = html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: format!("Wallchart: {}", worksite_name),
                buttons: html! {
                    <SecondaryButton
                        hx_get=format!("/wallcharts/{}/locations/new-modal", &id)
                        hx_target="body"
                        hx_swap="beforeend"
                        hx_push_url=format!("/wallcharts/{}/locations/new", &id)
                    >
                        Add New Location
                    </SecondaryButton>
                    <SecondaryButton
                        tag="a"
                        href=format!("/wallcharts/{}/tags", &id)
                    >
                        Manage Tags
                    </SecondaryButton>
                    <PrimaryButton
                        onclick="alert('Coming soon!')"
                    >
                        Edit Worksite
                    </PrimaryButton>
                }
            }
        >
            <NotificationFlashes flashes=flashes.clone() />
            <div class="my-4">
                <p><em>Manage your worksite and more.</em></p>
                <div class="mt-8 flow-root">
                    <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                        <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                            <Wallchart worksite=worksite/>
                        </div>
                    </div>
                </div>
            </div>
        </PageLayout>
    };

    (flashes, Html(html))
}
