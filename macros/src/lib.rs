#[macro_export]
macro_rules! html_attrs {
    ($pub:vis struct $name:ident { $(#[$x:meta] $fpub:vis $field:ident : $type:ty,)* }) => {
        #[props]
        $pub struct $name {
            #[builder(default)]
            id: String,

            #[builder(default)]
            class: String,

            #[builder(default)]
            onclick: String,

            #[builder(default)]
            role: String,

            #[builder(default)]
            aria_orientation: String,

            #[builder(default)]
            aria_labelledby: String,

            #[builder(default)]
            tabindex: String,

            #[builder(default)]
            data: std::collections::HashMap<&'static str, String>,

            #[builder(default=String::from("div"))]
            tag: String,

            $(
                #[$x]
                $fpub $field: $type,
            )*
        }
    };
}
