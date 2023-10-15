use axum::response::Html;
use rscx::html;

pub async fn get_wallchart_page() -> Html<String> {
    Html(html! {
        <p>"Hello, world!"</p>
    })
}
