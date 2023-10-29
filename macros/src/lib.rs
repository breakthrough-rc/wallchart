#[macro_export]
macro_rules! html_attrs {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $(#[builder($($propsbuilder:meta),+)] $fpub:vis $field:ident : $type:ty,)* }) => {
        #[derive($($derive),*)]
        #[props]
        $pub struct $name {
            #[builder(default)]
            class: String,

            $(#[builder(default)] $fpub $field : $type,)*
            //$(#[builder($($propsbuilder),+)] $fpub $field : $type,)*
        }
    }
}
