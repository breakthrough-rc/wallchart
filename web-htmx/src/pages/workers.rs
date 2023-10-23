
use axum::response::Html;
use axum::{
    extract::{self, State},
    response::IntoResponse,
};
use rscx::html;

use crate::components::wallchart;
use crate::{
    page::PageLayout,
};

pub async fn get_workers_new_page(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(
        String,
        String,
        String,
    )>,
) -> Html<String> {
    Html(html! {
        <PageLayout>
            <h3>New Worker</h3>
            <form hx-post=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", wallchart_id, location_id, shift_id)>
            <div class="form-group">
                <label>First Name</label>
                <input type="text" class="form-control" name="firstName">
            </div>
            <div class="form-group">
                <label>Last Name</label>
                <input type="text" class="form-control" name="lastName">
            </div>
            <button class="btn btn-default">Submit</button>
            </form>
        </PageLayout>
    })
}

pub async fn post_workers_new_page(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(
        String,
        String,
        String,
    )>,
) -> Html<String> {
    Html(html! {
        <div>Hi</div>
    })
}