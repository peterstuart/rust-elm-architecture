extern crate proc_macro;
use proc_macro2::TokenStream;
use syn::{Error, Expr, Lit, LitStr};

pub fn str_literal(input: Expr) -> Result<LitStr, TokenStream> {
    let lit = match input {
        Expr::Lit(lit) => Ok(lit.lit),
        _ => Err(syn::Error::new_spanned(input, "must be a str literal").to_compile_error()),
    }?;

    match lit {
        Lit::Str(str) => Ok(str),
        _ => Err(Error::new_spanned(lit, "must be a str literal").to_compile_error()),
    }
}
