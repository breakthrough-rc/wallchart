use super::appshell::AppShell;
pub use super::appshell::PageHeader;
use rscx::{component, html, props};
use web_client::server::{modal::ModalLiveRegion, notification::NotificationLiveRegion};
use web_client::HtmlLayout;

#[props]
pub struct PageLayoutProps {
    #[builder(setter(into), default = "Page".into())]
    header: PageHeader,

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
                    // Use unminified source for debugging.
                    // <script src="https://unpkg.com/htmx.org@1.9.9/dist/htmx.js"></script>
                    <script
                        src="https://unpkg.com/htmx.org@1.9.9"
                        integrity="sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX"
                        crossorigin="anonymous"
                    ></script>
                    <script src="https://unpkg.com/htmx.org/dist/ext/loading-states.js"></script>
                }
            }
        >
            <AppShell header=props.header>
                <main hx-ext="loading-states">
                    {props.children}
                </main>
            </AppShell>
            <NotificationLiveRegion />
            <ModalLiveRegion />
            <script>{
                r#"
                htmx.on("htmx:sendError", function() {
                    YcControls.showErrorNotification("Network Error!");
                });                

                htmx.on("htmx:responseError", function(error) {
                    YcControls.showErrorNotification(
                        error.detail.xhr.responseText || "Unknown error"
                    );
                });
                "#
                // document.addEventListener("htmx:confirm", function(e) {
                //     e.preventDefault();
                //     YcControls.confirm({
                //         title: e.target.getAttribute("hx-confirm"),
                //         message: e.target.dataset.confirmMessage,
                //         actionConfirmed: function() {
                //             e.detail.issueRequest(true);
                //         },
                //     });
                // })

            }</script>
        </HtmlLayout>
    }
}
