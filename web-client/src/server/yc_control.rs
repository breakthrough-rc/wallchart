use super::html_element::{html_attrs, Attrs, HtmlElement};
use rscx::{component, html, props};
use std::collections::HashMap;

html_attrs! {
    pub struct YcControlProps {
        #[builder(default)]
        control: String,

        #[builder(default)]
        children: String,
    }
}

#[component]
pub fn YcControl(props: YcControlProps) -> String {
    let original_props = props.clone();
    html! {
        <HtmlElement
            data=HashMap::from([("yc-control", props.control)])
            attrs=Attrs::from(original_props)
        >
            {props.children}
            <script>"YcControls.attach(document.currentScript.parentElement)"</script>
        </HtmlElement>
    }
}

html_attrs! {
    pub struct ToggleProps {
        #[builder(default)]
        children: String,
    }
}

#[component]
pub fn Toggle(props: ToggleProps) -> String {
    let original_props = props.clone();
    html! {
        <YcControl
            control="toggle".into()
            attrs=Attrs::from(original_props)
        >
            {props.children}
        </YcControl>
    }
}
