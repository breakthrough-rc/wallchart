
use axum::response::Html;
use rscx::html;

use crate::{
    page::PageLayout,
};

pub async fn get_workers_new_page() -> Html<String> {
    Html(html! {
        <PageLayout>
            <h3>New Worker</h3>
            <form hx-post="/workers">
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
