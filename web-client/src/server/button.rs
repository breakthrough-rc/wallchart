use super::html_element::HtmlElement;
use rscx::{component, html, props};
use web_macros::*;

#[derive(Clone)]
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[html_element]
pub struct PrimaryButtonProps {
    children: String,

    #[builder(default = ButtonSize::Md)]
    size: ButtonSize,

    #[builder(setter(into), default=String::from("button"))]
    tag: String,
}

#[component]
pub fn PrimaryButton(props: PrimaryButtonProps) -> String {
    html! {
        <HtmlElement
            tag=props.tag
            class={
                let class = match props.size {
                    ButtonSize::Xs => "rounded bg-indigo-600 px-2 py-1 text-xs font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    ButtonSize::Sm => "rounded bg-indigo-600 px-2 py-1 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    ButtonSize::Md => "rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    ButtonSize::Lg => "rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    ButtonSize::Xl => "rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                };
                format!("{} {}", class, props.class).trim()
            }
            attrs=spread_attrs!(props | omit(class)).set("type", "button".into())
        >
            {props.children}
        </HtmlElement>
    }
}
