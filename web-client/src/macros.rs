// MODULE IS DEPRECATED. Please use/see the `html_element` macro in the `web-macro` crate.

/*
 To add an attribute to the html_attrs macro,
   - add a field to the struct
   - add the attribute to the `to_hashmap` function.
*/

#[macro_export]
macro_rules! html_attrs {
    ($pub:vis struct $name:ident { $(#[$x:meta] $fpub:vis $field:ident : $type:ty,)* }) => {
        #[props]
        #[derive(Clone)]
        #[allow(dead_code)]
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
            attrs: ::web_client::server::attrs::Attrs,

            #[builder(default)]
            data: std::collections::HashMap<&'static str, String>,

            #[builder(default=String::from("div"))]
            tag: String,

            $(
                #[$x]
                $fpub $field: $type,
            )*
        }

        impl $name {
            fn html_attrs_to_hashmap(&self) -> std::collections::HashMap<&'static str, String> {
                let mut map = std::collections::HashMap::new();

                map.insert("id", self.concat_attribute(&self.id, self.attrs.get("id")));
                map.insert("class", self.concat_attribute(&self.class, self.attrs.get("class")));
                map.insert("onclick", self.concat_attribute(&self.onclick, self.attrs.get("onclick")));
                map.insert("role", self.concat_attribute(&self.role, self.attrs.get("role")));
                map.insert("aria-orientation", self.concat_attribute(&self.aria_orientation, self.attrs.get("aria-orientation")));
                map.insert("aria-labelledby", self.concat_attribute(&self.aria_labelledby, self.attrs.get("aria_labelledby")));
                map.insert("tabindex", self.concat_attribute(&self.tabindex, self.attrs.get("tabindex")));

                map
            }

            fn concat_attribute(&self, field_value: &str, attribute_value: Option<&String>) -> String {
                let mut values = vec![];

                if !field_value.is_empty() {
                    values.push(field_value.trim());
                }

                if let Some(value) = attribute_value {
                    values.push(value.trim());
                }

                values.join(" ")
            }
        }

        impl From<$name> for ::web_client::server::attrs::Attrs {
            fn from(html_props: $name) -> Self {
                ::web_client::server::attrs::Attrs::from(html_props.html_attrs_to_hashmap())
            }
        }
    };
}
