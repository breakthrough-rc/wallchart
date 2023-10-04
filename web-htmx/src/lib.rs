use axum::{
    response::Html,
    Router, 
    routing::get,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_home))    
}

async fn get_home() -> Html<String> {
    Html(
        "<marquee>ONE SMALL STEP FOR AN HTML_ASSHOLE, ONE GIANT LEAP FOR HTML_ASSHOLEKIND</marquee>".into()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
