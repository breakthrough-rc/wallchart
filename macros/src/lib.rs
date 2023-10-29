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

                map
            }
        }
    };
}
