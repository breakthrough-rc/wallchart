pub extern crate rscx;
use rscx::rscx_macros;
use rscx::{component, html, props};

macro_rules! html_pb {
    () => {
        #[builder(default)]
    };
}

#[macro_export]
macro_rules! html_attrs {
    // (@loop, $($x:meta),+) => {
    //     #[builder($($x)+)]
    // };
    (@loop, $x:meta) => {
        #[builder(default)]
    };

    (@loop, $($x:meta),+) => {
        #[builder(default)]
    };

    ($pub:vis struct $name:ident { $(#[$x:meta] $fpub:vis $field:ident : $type:ty,)* }) => {
        #[props]
        $pub struct $name {
            #[builder(default)]
            class: String,

            #[builder(default)]
            id: String,

            #[builder(default="div".to_string())]
            tag: String,

            $(
                #[$x]
                $fpub $field: $type,
            )*
        }
    };

    // ($pub:vis struct $name:ident { $(#[builder($($x:meta),+)] $fpub:vis $field:ident : $type:ty,)* }) => {
    //     #[props]
    //     $pub struct $name {
    //         #[builder(default)]
    //         class: String,

    //         //$(#[builder(default)] $fpub $field : $type,)*
    //         //$(#[builder($($propsbuilder),+)] $fpub $field : $type,)*

    //         $(
    //             #[builder(default)]
    //             $fpub $field: $type,
    //         )*
    //     }

    //     $(#[builder($($x),+)])*
    //     struct Test {}

    // };
}

/*
impl ::rscx::props::Props for SimpleElementProps {
        type Builder = SimpleElementPropsBuilder;
        fn builder() -> Self::Builder {
            SimpleElementProps::builder()
        }
    }
*/

/*
#[macro_export]
macro_rules! html_attrs {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $(#[builder($($propsbuilder:meta),+)] $fpub:vis $field:ident : $type:ty,)* }) => {
        #[derive($($derive),*)]
        #[props]
        $pub struct $name {
            #[builder(default)]
            class: String,

            //$(#[builder(default)] $fpub $field : $type,)*
            //$(#[builder($($propsbuilder),+)] $fpub $field : $type,)*
            $(
                #[builder($($propsbuilder),+)]
                $fpub $field : $type,
            )*
        }
    }
}
*/
