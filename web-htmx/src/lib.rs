#![feature(proc_macro_hygiene)]

use axum::{
    response::Html,
    Router, 
    routing::get,
};
use render::{component, rsx, html};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_home))    
}


#[component]
fn Heading<'title>(title: &'title str) {
  rsx! { <h1 class={"title"}>{title}</h1> }
}

async fn get_home() -> Html<String> {
    html! {
        <marquee>
            <Heading title={"hello!"}></Heading>
        </marquee>
    }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
