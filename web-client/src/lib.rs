use axum::Router;
use tower_http::services::ServeDir;
use rscx::{component, html, props};

pub fn routes() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("web-client/out"))
}

#[props]
pub struct HtmlLayoutProps {
    #[builder(setter(into), default = "Yall Chart".to_string())]
    head_title: String,

    #[builder(default)]
    head_links: String,

    #[builder(default)]
    head_scripts: String,    

    #[builder(default)]
    children: String,
}

#[component]
pub fn HtmlLayout(props: HtmlLayoutProps) -> String {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{props.head_title}</title>
                {props.head_links}
                {props.head_scripts}
            </head>
            <body>
                {props.children}
                <script src="/client/common.js"></script>
            </body>
        </html>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
