use axum::{
    response::Html,
    Router, 
    routing::get,
};
use rscx::{component, html, props};
use page::PageLayout;
use web_client::routes as client_routes;

pub mod livereload;
pub mod page;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_home))
        .route("/htmx", get(htmx_test))
        .nest_service("/client", client_routes())   
}

#[props]
struct WelcomeProps {
    #[builder(setter(into), default = "Welcome!".to_string())]
    title: String,

    #[builder(default)]
    children: String,
}

#[component]
fn Welcome(props: WelcomeProps) -> String {
    html! { 
        <h1 class="text-xl text-slate-600">{props.title}</h1>
        {props.children}
    }
}

async fn get_home() -> Html<String> {
    Html(html! {
        <PageLayout>
            <Welcome 
                title="Yall Ready for This?"
            >
                <marquee>
                    "I didn't think so!"
                </marquee>
                <button 
                    class="bg-slate-200 p-3 rounded-full" 
                    hx-get="/htmx" 
                    hx-swap="outerHTML"
                >
                    Click me!
                </button>
            </Welcome>
        </PageLayout>
    })
}

async fn htmx_test() -> Html<String> {
    Html("Is this the real life? Is this just fantasy?".into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
