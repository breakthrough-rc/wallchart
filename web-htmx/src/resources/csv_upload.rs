use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rscx::html;
use std::str::from_utf8;

use web_client::server::{card::Card, form::Button};

use crate::{
    components::{page::PageLayout, page_content::PageContent},
    routes,
    state::WebHtmxState,
};

pub fn csv_upload_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            routes::CSV_UPLOAD,
            get(get_csv_upload).post(post_csv_upload),
        )
        .with_state(state)
}

async fn get_csv_upload(State(_state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Upload a CSV">
            <p><em>Bulk upload data into your worksites.</em></p>
            <PageContent>
                <Card padded=true>
                    <form id="form" hx-encoding="multipart/form-data" hx-post=routes::csv_upload()>
                        <p>Select a file to upload.</p>
                        <input type="file" name="file" />
                        <Button kind="submit">Upload</Button>
                    </form>
                </Card>
            </PageContent>
        </PageLayout>
    })
}

async fn post_csv_upload(
    State(_state): State<WebHtmxState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut context: Vec<String> = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
        let data = from_utf8(&bytes).unwrap();
        context.push(data.to_string());
    }

    Html(html! {
        <PageLayout header="Upload a CSV">
            <p>You uploaded: {context.join(", ")}</p>
        </PageLayout>
    })
}
