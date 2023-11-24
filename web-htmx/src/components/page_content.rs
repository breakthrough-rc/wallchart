use rscx::{component, html, props};

#[props]
pub struct PageContentProps {
    children: String,
}

#[component]
pub fn PageContent(props: PageContentProps) -> String {
    html! {
        <div class="flex flex-col">
            <div class="mt-8 flow-root">
                <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                        {props.children}
                    </div>
                </div>
            </div>
        </div>
    }
}
