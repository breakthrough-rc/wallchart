use axum::{extract::State, response::Html};
use rscx::html;
use worksite_service::get_worksite::GetWorksiteInput;

use crate::{components::wallchart::Wallchart, page::PageLayout, state::WebHtmxState};

pub async fn get_wallchart_page(
    State(WebHtmxState { worksite_service }): State<WebHtmxState>,
) -> Html<String> {
    let worksite = worksite_service
        .get_worksite(GetWorksiteInput {
            id: "1".to_string(),
        })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    Html(html! {
        <PageLayout title="Wallchart">
            <div class="my-4">
                <Wallchart worksite=worksite/>
            </div>
        </PageLayout>
    })
}
