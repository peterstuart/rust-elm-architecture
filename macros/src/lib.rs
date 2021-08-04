extern crate proc_macro;

mod utils;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::Expr;

#[proc_macro]
pub fn element(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as Expr);
    let value = match utils::str_literal(input) {
        Ok(value) => value,
        Err(error) => return error.into(),
    };

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

    result.into()
}
