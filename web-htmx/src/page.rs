use rscx::{component, html, props};
use web_client::HtmlLayout;

#[props]
pub struct PageLayoutProps {
    #[builder(default = false)]
    partial: bool,

    #[builder(default)]
    children: String,
}

#[component]
pub fn PageLayout(props: PageLayoutProps) -> String {
    if props.partial {
        return props.children;
    }

    html! {
        <HtmlLayout
            head_scripts={
                html! {
                    <script
                        src="https://unpkg.com/htmx.org@1.9.5"
                        integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
                        crossorigin="anonymous"
                    ></script>
                    <script src="https://unpkg.com/htmx.org/dist/ext/loading-states.js"></script>
                }
        }
        >
            <header class="bg-slate-200 border-y border-b-indigo-500">
                <h1>Yall Chart</h1>
            </header>
            <main hx-ext="loading-states">
                {props.children}
            </main>
            <footer class="text-xs">
                <small>{"2023 &copy; Yall Chart"}</small>
            </footer>
            <script>{
                "
                htmx.on('htmx:sendError', function() {
                    YcControls.showErrorNotification('Network Error!');
                });                

                htmx.on('htmx:responseError', function(error) {
                    YcControls.showErrorNotification(
                        error.detail.xhr.responseText || 'Unknown error'
                    );
                });
                "
            }</script>
        </HtmlLayout>
    }
}
