use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rscx::html;
use serde::Deserialize;
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
                    <form id="form"
                        hx-encoding="multipart/form-data"
                        hx-post=routes::csv_upload()
                        hx-target="#form"
                    >
                        <p>Select a file to upload.</p>
                        <input type="file" name="file" />
                        <Button kind="submit">Upload</Button>
                    </form>
                </Card>
            </PageContent>
        </PageLayout>
    })
}

/**
* This is an example CSV row. We use this to construct all of the
* worksites/locations/shifts/workers as needed.
*
* There is an enourmous amount of room for improvement but this is a quick and
* dirty onboarding tool. We should improve this process as we encounter real
* life usecases.
*
* At the moment, this code assumes that worksites, locations, shifts etc. are names that
* are relatively unique (to their context) so we can create/reuse these resources
* as needed
*/
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WorkerRecord {
    worksite: String,
    location: String,
    shift1: String,
    shift2: String,
    shift3: String,
    name: String,
    email: String,
    phone: String,
}
async fn post_csv_upload(
    State(_state): State<WebHtmxState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut content: Vec<String> = vec![];
    /*
     * First we process the multipart input to get a vec of the file contents
     *
     * Multipart processing is based on the breaking the file up into arb chunks,
     * not line by line. So we have to reassemble the file contents from the chunks.
     */
    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
        let data = from_utf8(&bytes).unwrap();
        content.push(data.to_string());
    }

    /*
     * Now we have the file contents, we can process it as a CSV
     *
     * We combine the "chunks" into a single string and then deserialize row by row
     */
    let content: String = content.join("");
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let records: Vec<WorkerRecord> = rdr.deserialize().map(|result| result.unwrap()).collect();

    Html(html! {
        <p>Here is the data we received:</p>
        <pre>{format!("{:#?}", records)}</pre>
    })
}
