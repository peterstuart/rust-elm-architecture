extern crate proc_macro;

mod utils;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, Expr, Token};

fn from_result<F>(f: F) -> TokenStream
where
    F: FnOnce() -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream>,
{
    f().unwrap_or_else(|e| e).into()
}

#[proc_macro]
pub fn element(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as Expr);

    from_result(move || {
        let value = utils::str_literal(&input)?;

        let function_name = Ident::new(&value.value(), value.span());
        let node_name = value.value();

        let result = quote! {
          pub fn #function_name(attributes: Vec<Attribute<Message>>, children: Vec<Node<Message>>) -> Node<Message> {
            Node::Element(Element {
              name: #node_name.into(),
              attributes,
              children,
            })
          }
        };

        Ok(result)
    })
}

#[proc_macro]
pub fn attribute(items: TokenStream) -> TokenStream {
    attribute_macro(items, |function_name, attribute_name| {
        quote! {
            pub fn #function_name(value: &str) -> Self {
                Self::Text(#attribute_name.into(), value.into())
            }
        }
    })
}

#[proc_macro]
pub fn bool_attribute(items: TokenStream) -> TokenStream {
    attribute_macro(items, |function_name, attribute_name| {
        quote! {
            pub fn #function_name(value: bool) -> Self {
                Self::Bool(#attribute_name.into(), value)
            }
        }
    })
}

fn attribute_macro<FunctionGenerator>(
    items: TokenStream,
    function_generator: FunctionGenerator,
) -> TokenStream
where
    FunctionGenerator: FnOnce(&Ident, &String) -> proc_macro2::TokenStream,
{
    from_result(move || {
        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
        let exprs = parser
            .parse(items)
            .map_err(|error| error.to_compile_error())?;

        if exprs.len() > 2 {
            return Err(
                syn::Error::new_spanned(exprs, "must have 1 or 2 arguments").to_compile_error()
            );
        }

        let value = utils::str_literal(exprs.first().unwrap())?;

        let function_name = if exprs.len() == 2 {
            let custom_function_name = utils::str_literal(exprs.last().unwrap())?;
            Ident::new(&custom_function_name.value(), custom_function_name.span())
        } else {
            Ident::new(&value.value().replace("-", "_"), value.span())
        };

        let attribute_name = value.value();

        Ok(function_generator(&function_name, &attribute_name))
    })
}
