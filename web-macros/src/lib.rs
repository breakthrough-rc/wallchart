use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Expr, Fields,
    FieldsNamed, Ident, ItemStruct, Token,
};

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

        let attr_idents = create_attr_idents();

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

                #[builder(setter(into), default=String::from("div"))]
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
            #[derive(Clone)]
            #item
        });

        let attr_keys = HTML_ELEMENT_ATTRS.clone();
        tokens.extend(quote! {
            impl #name {
                fn html_attrs_to_hashmap(&self) -> std::collections::HashMap<&'static str, String> {
                    let mut map = std::collections::HashMap::new();

                    #(
                        map.insert(#attr_keys, web_client::concat_attribute(&self.#attr_idents, self.attrs.get(#attr_keys)));
                    )*

                    map
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

fn create_attr_idents() -> [syn::Ident; ATTRS_LEN] {
    HTML_ELEMENT_ATTRS.map(|attr| {
        let attr = attr.replace("-", "_");
        let attr = attr.as_str();

        syn::Ident::new(attr, Span::call_site())
    })
}

#[derive(Debug)]
struct AttrsSpread {
    props: Ident,
    omit: Punctuated<Expr, Comma>,
}

impl Parse for AttrsSpread {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let props: Ident = input.parse()?;

        input.parse::<Token![|]>()?;
        let omit: Expr = input.parse()?;

        if let Expr::Call(syn::ExprCall { func: _, args, .. }) = omit {
            Ok(AttrsSpread { props, omit: args })
        } else {
            panic!("Expected omit call in macro!");
        }
    }
}

#[proc_macro]
pub fn spread_attrs(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as AttrsSpread);
    let AttrsSpread { props, omit } = ast;

    let mut attr_keys = HTML_ELEMENT_ATTRS.clone().to_vec();
    let mut attr_idents = create_attr_idents().to_vec();

    for field in omit {
        match field {
            Expr::Path(path) => {
                let ident = path.path.get_ident().unwrap();
                if !HTML_ELEMENT_ATTRS.contains(&ident.to_string().replace("_", "-").as_str()) {
                    panic!(
                        "Cannot omit field {}. It doesn't exit in HtmlElementProps.",
                        ident
                    );
                }
                attr_keys.retain(|f| f != &ident.to_string());
                attr_idents.retain(|f| f.to_string() != ident.to_string());
            }
            _ => (),
        }
    }

    let gen = quote! {
        {
            let mut map = std::collections::HashMap::new();
            #(
                map.insert(#attr_keys, web_client::concat_attribute(&#props.#attr_idents, #props.attrs.get(#attr_keys)));
            )*

            ::web_client::server::attrs::Attrs::from(map)
        }
    };

    gen.into()
}
