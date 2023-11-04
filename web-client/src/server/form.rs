use super::html_element::HtmlElement;
use rscx::{component, html, props};
use web_macros::*;

#[html_element]
pub struct TextInputProps {
    #[builder(setter(into), default="text".into())]
    input_type: String,
}

#[component]
pub fn TextInput(props: TextInputProps) -> String {
    html! {
        <HtmlElement
            tag="input"
            id=props.name.clone()
            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            attrs=spread_attrs!(props | omit(id, class)).set("type", props.input_type)
        />
    }
}

#[html_element]
pub struct LabelProps {
    #[builder(setter(into))]
    for_input: String,
    children: String,
}

#[component]
pub fn Label(props: LabelProps) -> String {
    html! {
        <HtmlElement
            tag="label"
            class=format!("block text-sm font-medium leading-6 text-gray-900 {}", props.class).trim()
            attrs=spread_attrs!(props | omit(class)).set("for", props.for_input)
        >
            {props.children}
        </HtmlElement>
    }
}

#[html_element]
pub struct ButtonProps {
    #[builder(setter(into), default="button".into())]
    kind: String,
    children: String,
}

#[component]
pub fn Button(props: ButtonProps) -> String {
    let button_type = props.kind.clone();
    let css = match props.kind.as_str() {
        "submit" => "rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
        _ => "text-sm font-semibold leading-6 text-gray-900",
    };

    html! {
        <HtmlElement
            tag="button"
            class=css
            attrs=spread_attrs!(props | omit(class, name)).set("type", button_type)
        >
            {props.children}
        </HtmlElement>
    }
}
