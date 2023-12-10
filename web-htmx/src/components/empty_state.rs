use rscx::{component, html, props};
use web_client::server::html_element::HtmlElement;
use web_macros::{html_element, spread_attrs};

#[html_element]
pub struct EmptyStateProps {
    #[builder(setter(into))]
    button_label: String,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> String {
    html! {
        <HtmlElement
            tag="button"
            class="relative block w-full rounded-lg border-2 border-dashed border-gray-300 p-12 text-center hover:border-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            attrs=spread_attrs!(props | omit(class))
        >
            <svg class="mx-auto h-12 w-12 text-gray-400" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M4 12H20M12 4V20" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
            <span  class="mt-2 block text-sm font-semibold text-gray-900">{props.button_label}</span>
        </HtmlElement>
    }
}
