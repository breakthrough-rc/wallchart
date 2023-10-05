use axum::{
    response::Html,
    Router, 
    routing::get,
};
use rscx::{component, html, props};
use web_client::{
    routes as client_routes,
    HtmlLayout,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_home))
        .route("/htmx", get(htmx_test))
        .nest_service("/client", client_routes())   
}

#[props]
/// mark a struct with #[props] to use it as props in a component.
/// #[builder] can customize single props, marking them as option or setting a default value.
struct WelcomeProps {
    #[builder(setter(into), default = "Welcome!".to_string())]
    title: String,

    #[builder(default)]
    children: String,
}

#[component]
fn Welcome(props: WelcomeProps) -> String {
    html! { 
        <main>    
            <h1 class={"text-xl"}>{ props.title }</h1>
            { props.children }
        </main>
    }
}

async fn get_home() -> Html<String> {
    Html(html! {
        <HtmlLayout
            head_scripts={
                html! {
                    <script 
                        src="https://unpkg.com/htmx.org@1.9.5" 
                        integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO" 
                        crossorigin="anonymous"
                    ></script>
                }
            }
        >
            <Welcome 
                title="Yall Ready for This?"
            >
                <marquee>
                    "I didn't think so!"
                </marquee>
                <button class={"bg-slate-200 p-3 rounded-full"} hx-get="/htmx" hx-swap="outerHTML">Click me!</button>
            </Welcome>
        </HtmlLayout>
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
