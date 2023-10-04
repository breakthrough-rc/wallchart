use axum::{
    response::Html,
    Router, 
    routing::get,
};
use rscx::{component, html, props};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_home))    
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
        <h1>{ props.title }</h1>
        { props.children }
    </main>
 }
}

async fn get_home() -> Html<String> {
    Html(html! {
        <!DOCTYPE html>
        <html>
            <body>
                <Welcome title="Yall Ready for This?">
                    <marquee>
                        "I didn't think so!"
                    </marquee>
                </Welcome>
            </body>
        </html>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
