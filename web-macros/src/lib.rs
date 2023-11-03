use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, Fields, FieldsNamed, ItemStruct};

const ATTRS_LEN: usize = 7;
const HTML_ELEMENT_ATTRS: [&str; ATTRS_LEN] = [
    "id",
    "class",
    "onclick",
    "role",
    "aria-orientation",
    "aria-labelledby",
    "tabindex",
];

#[proc_macro_attribute]
pub fn html_element(_: TokenStream, input: TokenStream) -> TokenStream {
    let html_element = parse_macro_input!(input as HtmlElementStruct);
    quote! { #html_element }.to_token_stream().into()
}

struct HtmlElementStruct {
    name: syn::Ident,
    item: ItemStruct,
}

impl Parse for HtmlElementStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<ItemStruct>()?;
        let name = item.ident.clone();

        Ok(HtmlElementStruct { name, item })
    }
}

impl ToTokens for HtmlElementStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let original_item: &ItemStruct = &self.item;
        let item = self.item.clone();

        let original_fields = match item.fields {
            Fields::Named(named) => named.named,
            _ => panic!("not named fields"),
        };

        let attr_idents = HTML_ELEMENT_ATTRS.map(|attr| {
            let attr = attr.replace("-", "_");
            let attr = attr.as_str();

            syn::Ident::new(attr, proc_macro2::Span::call_site())
        });

        let fields = quote! {
            {
                #original_fields

                #(
                    #[builder(setter(into), default)]
                    #attr_idents: String,
                )*

                #[builder(default)]
                attrs: ::web_client::server::attrs::Attrs,

                #[builder(default)]
                data: std::collections::HashMap<&'static str, String>,

                #[builder(default=String::from("div"))]
                tag: String,
            }
        };
        let fields: FieldsNamed = syn::parse_quote! { #fields };
        let fields = Fields::Named(fields);

        let item = &ItemStruct {
            fields: fields.into(),
            ..original_item.clone()
        };

        tokens.extend(quote! {
            #[props]
            #item
        });

        let attr_keys = HTML_ELEMENT_ATTRS.clone();
        tokens.extend(quote! {
            impl #name {
                fn html_attrs_to_hashmap(&self) -> std::collections::HashMap<&'static str, String> {
                    let mut map = std::collections::HashMap::new();

                    #(
                        map.insert(#attr_keys, self.concat_attribute(&self.#attr_idents, self.attrs.get(#attr_keys)));
                    )*

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

            impl From<#name> for ::web_client::server::attrs::Attrs {
                fn from(html_props: #name) -> Self {
                    ::web_client::server::attrs::Attrs::from(html_props.html_attrs_to_hashmap())
                }
            }
        });
    }
}
