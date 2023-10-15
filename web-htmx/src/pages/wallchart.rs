use axum::response::Html;
use rscx::html;

use crate::{
    components::wallchart::{Wallchart, Worksite},
    page::PageLayout,
};

pub async fn get_wallchart_page() -> Html<String> {
    let worksite = Worksite {
        id: "1".to_string(),
        name: "Dunder Mifflin".to_string(),
        locations: vec![],
    };
    Html(html! {
    <PageLayout>
        <div class="my-4">
            <Wallchart worksite=worksite/>
        </div>
    </PageLayout>
    })
}
