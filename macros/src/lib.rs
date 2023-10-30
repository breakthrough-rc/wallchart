use std::collections::HashMap;
/*
 To add an attribute to the html_attrs macro,
   - add a field to the struct
   - add the attribute to the `to_hashmap` function.
*/

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
            attrs: ::macros::Attrs,

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

                map.insert("id", self.id.clone());
                map.insert("class", self.class.clone());
                map.insert("onclick", self.onclick.clone());
                map.insert("role", self.role.clone());
                map.insert("aria-orientation", self.aria_orientation.clone());
                map.insert("aria-labelledby", self.aria_labelledby.clone());
                map.insert("tabindex", self.tabindex.clone());

                map.extend(self.attrs.to_hashmap());
                map
            }
        }

        impl From<$name> for Attrs {
            fn from(html_props: $name) -> Self {
                Attrs::from(html_props.html_attrs_to_hashmap())
            }
        }
    };
}

#[derive(Default)]
pub struct Attrs {
    values: std::collections::HashMap<&'static str, String>,
    omit: Vec<&'static str>,
}
impl Attrs {
    pub fn omit(&self, fields_to_omit: Vec<&'static str>) -> Self {
        Self {
            values: self.values.clone(),
            omit: fields_to_omit,
        }
    }
    pub fn to_hashmap(&self) -> std::collections::HashMap<&'static str, String> {
        let mut hashmap = self.values.clone();

        for field in &self.omit {
            hashmap.remove(field);
        }

        hashmap
    }
}
impl From<HashMap<&'static str, String>> for Attrs {
    fn from(html_attrs: HashMap<&'static str, String>) -> Self {
        Self {
            values: html_attrs,
            omit: vec![],
        }
    }
}
