use axum::{
    extract::{Multipart, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use http::StatusCode;
use rscx::{html, CollectFragment};

use std::str::from_utf8;
use worksite_service::csv_upload::CsvUploadInput;

use web_client::server::{
    alert::{Alert, AlertKind},
    card::Card,
    form::{Button, FileInput, Label},
    headers::SecondaryHeader,
};

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
        <PageLayout header="Import">
            <PageContent title="Bulk upload data into your worksites.">
                <Card padded=true>
                    <div class="flex flex-col gap-6">
                        <SecondaryHeader title="CSV Uploader" />
                        <Alert
                            title="You can upload a CSV to create worksites, locations, shifts and workers."
                            kind=AlertKind::Info
                        >
                            <div class="flex flex-col gap-6 mb-6">
                                <p>
                                    "The CSV requires a header row, and then each following row describes a worker for a worksite, and their shift assignments for a single location."
                                </p>
                                <p>
                                    "The shift columns are optional. If a worker is present in multiple locations/worksites, including those as extra rows."
                                </p>
                                <p>
                                    "This upload will only produce completely new worksites, and the names of the worksites, locations, shifts, and workers will be used to decide uniqueness within a single CSV upload batch."
                                </p>
                            </div>
                        </Alert>
                        <section>
                            <h2>"📂 See the following csv example:"</h2>
                            <pre class="my-2 p-6 whitespace-pre-line bg-gray-200 rounded">
                                <b>"worksite,location,shift1,shift2,shift3,first_name,last_name,email"</b><br />
                                "Dunder Miflin,Office,Day,,,Jim,Halpert,jim@dundermiflin.com
                                Dunder Miflin,Warehouse,Day,Night,,Doug,Worker,doug@dundermiflin.com
                                Office Max,Office,Day,,,Jay,Hackett,jay@officemax.com"
                            </pre>
                        </section>
                        <form id="form"
                            hx-encoding="multipart/form-data"
                            hx-post=routes::csv_upload()
                            hx-target="#form"
                        >
                            <Label for_input="file">
                                File upload
                            </Label>
                            <FileInput
                                id="file"
                                name="file"
                                file_hint_message="CSV up to 10MB"
                                accept=".csv"
                            />
                            <Button class="mt-4" kind="submit">
                                Submit
                            </Button>
                        </form>
                    </div>
                </Card>
            </PageContent>
        </PageLayout>
    })
}

async fn post_csv_upload(
    State(state): State<WebHtmxState>,
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

    let result = state
        .worksite_service
        .csv_upload(CsvUploadInput { csv_input: content })
        .await;

    match result {
        Ok(worksites) => Html(html! {
            <p>Your new worksites!</p>
            {
                worksites.into_iter().map(|worksite| {
                    html! {
                        <a href=routes::worksite(&worksite.id)>"Wallchart - " {worksite.name}</a>
                    }
                }).collect_fragment()
            }
        })
        .into_response(),
        Err(e) => match e {
            worksite_service::csv_upload::CsvUploadFailure::ParseFailure(parse_failure) => {
                (StatusCode::BAD_REQUEST, parse_failure).into_response()
            }
            worksite_service::csv_upload::CsvUploadFailure::Unknown(e) => {
                (StatusCode::BAD_REQUEST, e).into_response()
            }
        },
    }
}
