use super::nav::Nav;
use rscx::{component, html, props};
use web_client::server::page_header::PageHeaderToolbar;

pub enum PageHeader {
    None,
    Title(String),
    Toolbar { title: String, buttons: String },
}

impl Default for PageHeader {
    fn default() -> Self {
        Self::None
    }
}

impl From<String> for PageHeader {
    fn from(s: String) -> Self {
        Self::Title(s)
    }
}
impl From<&str> for PageHeader {
    fn from(s: &str) -> Self {
        Self::Title(s.to_string())
    }
}

#[props]
pub struct AppShellProps {
    #[builder(default)]
    header: PageHeader,

    #[builder(default)]
    children: String,
}

#[component]
pub fn AppShell(props: AppShellProps) -> String {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let is_logged_in = ctx.logged_in;

    html! {
        <div class="min-h-full" data-yc-app>
            { if is_logged_in { html! { <Nav /> } } else { html! {} } }
            <MainContent header=props.header>
                {props.children}
            </MainContent>
        </div>
    }
}

#[props]
pub struct MainContentProps {
    #[builder(default)]
    header: PageHeader,

    #[builder(default)]
    children: String,
}

#[component]
fn MainContent(props: MainContentProps) -> String {
    html! {
        <div class="py-10">
            {
                match props.header {
                    PageHeader::None => html! {},
                    PageHeader::Title(title) => html! {
                        <header class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                            <h1 class="text-3xl font-bold leading-tight tracking-tight text-gray-900">{title}</h1>
                        </header>
                    },
                    PageHeader::Toolbar { title, buttons } => html! {
                        <PageHeaderToolbar
                            class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8"
                            title=title
                            buttons=buttons
                        />
                    },
                }
            }
            <main>
                <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
                    {props.children}
                </div>
            </main>
        </div>
    }
}
