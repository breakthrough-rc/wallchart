use axum::response::Html;
use rscx::html;

use crate::{components::wallchart::Wallchart, page::PageLayout};

pub async fn get_wallchart_page() -> Html<String> {
    Html(html! {
    <PageLayout>
        <Wallchart />
    </PageLayout>
    })
}
