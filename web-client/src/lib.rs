extern crate self as web_client; // Allows web_client crate to import from itself when expanding macros.
use axum::Router;
use once_cell::sync::Lazy;
use rscx::{component, html, props};
use server::notification::NotificationLiveRegion;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::services::ServeDir;

pub mod macros;
pub mod server;

pub fn routes() -> Router {
    Router::new().nest_service("/", ServeDir::new("web-client/out"))
}

// TEMP HACK! Used to bust cache on client scripts and stylesheets.
// @TODO! Get hash of each build file and use that.
static TS: Lazy<u128> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
});

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
                <link href={format!("/client/common.css?ts={}", *TS)} rel="stylesheet" />
                <script>{
                    "window.YcControls = {
                        attachOnReadyQueue: [],
                        attach: function(element) {
                            this.attachOnReadyQueue.push(element);
                        }
                    };"
                }</script>
                {props.head_links}
                {props.head_scripts}
            </head>
            <body>
                {props.children}
                <NotificationLiveRegion />
                <script src={format!("/client/common.js?ts={}", *TS)}></script>
            </body>
        </html>
    }
}

pub fn concat_attribute(field_value: &str, attribute_value: Option<&String>) -> String {
    let mut values = vec![];

    if !field_value.is_empty() {
        values.push(field_value.trim());
    }

    if let Some(value) = attribute_value {
        values.push(value.trim());
    }

    values.join(" ")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
