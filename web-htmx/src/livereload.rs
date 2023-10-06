use axum::body::Body;
use http::Request;
use tower_livereload::{LiveReloadLayer, predicate::Predicate};

// Do not support livereload on htmx requests
// This prevents browser from crashing due to too many livereload event listeners on the page.
#[derive(Copy, Clone, Debug)]
pub struct DoNotReloadOnPartialHtmls;

impl<T> Predicate<Request<T>> for DoNotReloadOnPartialHtmls {
    fn check(&mut self, request: &Request<T>) -> bool {
        !request.headers().contains_key("Hx-Request")
    }
}  

pub fn layer() -> LiveReloadLayer<DoNotReloadOnPartialHtmls> {
    LiveReloadLayer::new()
        .request_predicate::<Body, DoNotReloadOnPartialHtmls>(DoNotReloadOnPartialHtmls)
}
