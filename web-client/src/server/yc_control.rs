use super::html_element::HtmlElement;
use rscx::{component, html, props};
use std::collections::HashMap;
use web_macros::*;

#[html_element]
pub struct YcControlProps {
    #[builder(setter(into), default)]
    control: String,

    #[builder(default)]
    children: String,
}

#[component]
pub fn YcControl(props: YcControlProps) -> String {
    html! {
        <HtmlElement
            data=HashMap::from([("yc-control", props.control)])
            attrs=spread_attrs!(props)
        >
            {props.children}
            <script>"YcControls.attach(document.currentScript.parentElement);"</script>
        </HtmlElement>
    }
}

#[html_element]
pub struct ToggleProps {
    #[builder(default)]
    children: String,
}

#[component]
pub fn Toggle(props: ToggleProps) -> String {
    html! {
        <YcControl
            control="toggle"
            attrs=spread_attrs!(props)
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
                    (function(callerScript) {{
                        YcControls.onReady(function() {{
                            YcControls.{};
                        }});
                    }}(document.currentScript));
                "#,
                props.call,
            )}
        </script>
    }
}
