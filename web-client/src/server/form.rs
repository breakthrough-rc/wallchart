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
pub struct SelectProps {
    children: String,
}

#[component]
pub fn Select(props: SelectProps) -> String {
    html! {
        <HtmlElement
            tag="select"
            id=props.name.clone()
            class=format!("block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:max-w-xs sm:text-sm sm:leading-6 {}", props.class).trim()
            attrs=spread_attrs!(props | omit(id, class))
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
            class=format!("{} {}", css, props.class).trim()
            attrs=spread_attrs!(props | omit(class, name)).set("type", button_type)
            data=props.data
        >
            {props.children}
        </HtmlElement>
    }
}

// FormLayouts ////////////////////////////////////////////////

#[html_element]
pub struct GridLayoutProps {
    children: String,
}

#[component]
pub fn GridLayout(props: GridLayoutProps) -> String {
    html! {
        <HtmlElement
            tag="div"
            class=format!("grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6 {}", props.class).trim()
            attrs=spread_attrs!(props | omit(class))
        >
            {props.children}
        </HtmlElement>
    }
}

#[derive(Clone)]
pub enum CellSpan {
    Size(usize),
    Full,
}

impl From<usize> for CellSpan {
    fn from(size: usize) -> Self {
        CellSpan::Size(size)
    }
}

#[html_element]
pub struct GridCellProps {
    children: String,

    #[builder(setter(into), default=CellSpan::Full)]
    span: CellSpan,

    #[builder(default = 0)]
    start: usize,
}

#[component]
pub fn GridCell(props: GridCellProps) -> String {
    html! {
        <HtmlElement
            tag="div"
            class={
                let mut classes = Vec::new();

                // For now hardcode this layout of cells (col w/ .5rem gap)
                // If we have other cell layouts, we can create new enum
                classes.push("flex flex-col gap-2".to_string());

                classes.push(match props.span {
                    // generates classes (for tailwind) in tailwind.config.js safelist
                    CellSpan::Size(size) => format!("sm:col-span-{}", size),
                    CellSpan::Full => "sm:col-span-full".to_string(),
                });

                if props.start > 0 {
                    // generates classes (for tailwind) in tailwind.config.js safelist
                    classes.push(format!("sm:col-start-{}", props.start));
                }

                if !props.class.is_empty() {
                    classes.push(props.class);
                }

                classes.join(" ")
            }
            attrs=spread_attrs!(props | omit(class))
        >
            {props.children}
        </HtmlElement>
    }
}
