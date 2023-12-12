use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rscx::html;

use web_client::server::{card::Card, headers::SecondaryHeader};

use crate::{
    components::{page::PageLayout, page_content::PageContent},
    routes,
};

pub fn support_routes() -> Router {
    Router::new().route(routes::SUPPORT, get(get_support))
}

async fn get_support() -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Support">
            <PageContent>
                <div class="flex flex-wrap gap-4">
                    <Card padded=true class="w-3/4 sm:w-[400px]">
                        <SecondaryHeader title="📢 Feedback" />
                        <div class="mt-4">
                            <p>
                                Want a new feature or improvement?
                            </p>
                            <p>
                                <a class="text-indigo-600 hover:text-indigo-900" href="https://github.com/breakthrough-rc/wallchart/issues/new">Create a new issue</a>
                                " or "
                                <a class="text-indigo-600 hover:text-indigo-900" href="https://github.com/breakthrough-rc/wallchart/issues">browse existing issues</a>
                            </p>
                        </div>
                    </Card>

                    <Card padded=true class="w-3/4 sm:w-[400px]">
                        <SecondaryHeader title="✉️ Contact Us" />
                        <div class="mt-4">
                            <p>
                                Have a question or just generally want to chat?
                            </p>
                            <p>
                                <a class="text-indigo-600 hover:text-indigo-900" href="https://discord.gg/ahZvy2G9fY">Talk to us on our Discord!</a>
                            </p>
                        </div>
                    </Card>
                </div>
            </PageContent>
        </PageLayout>
    })
}
