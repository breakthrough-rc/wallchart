use super::attrs::Attrs;
use super::html_element::HtmlElement;
use crate::html_attrs;
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
            <script>"YcControls.attach(document.currentScript.parentElement);"</script>
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
    let attrs = props.html_attrs_to_hashmap();

    html! {
        <YcControl
            control="toggle".into()
            attrs=Attrs::from(attrs)
        >
            {props.children}
        </YcControl>
    }
}

#[props]
pub struct YcControlJsApiProps {
    #[builder(setter(into))]
    call: String,
}

#[component]
pub fn YcControlJsApi(props: YcControlJsApiProps) -> String {
    html! {
        <script>
            {format!(
                r#"
                    YcControls.onReady(function() {{
                        YcControls.{};
                    }});
                "#,
                props.call,
            )}
        </script>
    }
}
