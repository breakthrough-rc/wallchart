use super::nav::Nav;
use rscx::{component, html, props};

#[props]
pub struct AppShellProps {
    #[builder(default)]
    title: String,

    #[builder(default)]
    children: String,
}

#[component]
pub fn AppShell(props: AppShellProps) -> String {
    html! {
        <div class="min-h-full">
            <Nav title=props.title.clone() />
            <MainContent title=props.title.clone()>
                {props.children}
            </MainContent>
        </div>
    }
}

#[props]
pub struct MainContentProps {
    #[builder(default)]
    title: String,

    #[builder(default)]
    children: String,
}

#[component]
fn MainContent(props: MainContentProps) -> String {
    html! {
        <div class="py-10">
            <header>
                <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                    <h1 class="text-3xl font-bold leading-tight tracking-tight text-gray-900">{props.title}</h1>
                </div>
            </header>
            <main>
                <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
                    {props.children}
                </div>
            </main>
        </div>
    }
}
