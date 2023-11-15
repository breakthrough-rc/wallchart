use super::appshell::AppShell;
use rscx::{component, html, props};
use web_client::server::notification::NotificationLiveRegion;
use web_client::HtmlLayout;

#[props]
pub struct PageLayoutProps {
    #[builder(setter(into), default = "Page".into())]
    title: String,

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
                    // For debugging use this -> <script src="https://unpkg.com/htmx.org@1.9.8/dist/htmx.js"></script>
                    <script
                        src="https://unpkg.com/htmx.org@1.9.5"
                        integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
                        crossorigin="anonymous"
                    ></script>
                    <script src="https://unpkg.com/htmx.org/dist/ext/loading-states.js"></script>
                }
            }
        >
            <AppShell title=props.title>
                <main hx-ext="loading-states">
                    {props.children}
                </main>
            </AppShell>
            <NotificationLiveRegion />
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
