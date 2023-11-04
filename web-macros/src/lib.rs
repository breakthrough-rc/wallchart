use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Expr, Fields,
    FieldsNamed, Ident, ItemStruct, Token, ExprCall,
};

const ATTRS_LEN: usize = 9;
const HTML_ELEMENT_ATTRS: [&str; ATTRS_LEN] = [
    "id",
    "class",
    "onclick",
    "role",
    "aria-orientation",
    "aria-labelledby",
    "tabindex",
    "name",
    "autocomplete",
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

                    // Check for special case html attributes that are not part of HtmlElementProps
                    if let Some(for_input) = self.attrs.get("for") {
                        map.insert("for", for_input.to_string());
                    }
                    if let Some(for_input) = self.attrs.get("type") {
                        map.insert("type", for_input.to_string());
                    }                    

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
    transforms: Vec<ExprCall>,
}

impl Parse for AttrsSpread {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut transforms: Vec<ExprCall> = Vec::new();

        let props: Ident = input.parse()?;

        while !input.is_empty() { 
            input.parse::<Token![|]>()?;

            let call: ExprCall = input.parse()?;
            transforms.push(call);
        }

        Ok(AttrsSpread { props, transforms })
    }
}

// Right now we only support omit transform in spread_attr!
enum Transformer {
    Omit(Punctuated<Expr, Comma>),
}

#[proc_macro]
pub fn spread_attrs(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as AttrsSpread);
    let AttrsSpread { props, transforms } = ast;

    let mut attr_keys = HTML_ELEMENT_ATTRS.clone().to_vec();
    let mut attr_idents = create_attr_idents().to_vec();

    for syn::ExprCall { func, args, .. } in transforms {
        let transform = match *func  {
            Expr::Path(path) if path.path.is_ident("omit") => Transformer::Omit(args),
            _ => panic!("Expected a pipe transform. Example: omit(field, ..)"),
        };

        transform_attrs(transform, &mut attr_keys, &mut attr_idents);
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

fn transform_attrs(transform: Transformer, attr_keys: &mut Vec<&str>, attr_idents: &mut Vec<Ident>) {
    match transform {
        Transformer::Omit(args) => {
            for arg in args {
                if let Expr::Path(path) = arg {
                    let ident = path.path.get_ident().unwrap();
                    if !HTML_ELEMENT_ATTRS.contains(&ident.to_string().replace("_", "-").as_str()) {
                        panic!(
                            "Cannot omit field {}. It doesn't exit in HtmlElementProps.",
                            ident,
                        );
                    }
                    attr_keys.retain(|f| f != &ident.to_string());
                    attr_idents.retain(|f| f.to_string() != ident.to_string());
                }
            }
        },
    }
}
