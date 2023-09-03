use core::iter::Peekable;

use into_gen::to_intos;
use parse::parse_enum;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::proc_macro_error;

mod parse;
mod struct_gen;
mod into_gen;
mod wo_vec;
use struct_gen::to_enum;
use wo_vec::WoVec;

struct Varient {
    id: Ident,
    sub_vars: Vec<&'static MyEnum>,
}

struct MyEnum {
    name: Ident,
    sub_enums: Vec<&'static MyEnum>,
    varients: Vec<Varient>,
}

#[proc_macro]
#[proc_macro_error]
pub fn flattenum(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t_tokens: proc_macro2::TokenStream = tokens.into();
    let mut m_tokens: Peekable<<TokenStream as IntoIterator>::IntoIter> =
        t_tokens.into_iter().peekable();
    let mut enums = WoVec::new();
    while m_tokens.peek().is_some() {
        let en = parse_enum(&mut m_tokens, &enums);
        enums.push(en);
    }
    let len = enums.len();
    let enum_streams = unsafe { enums.get_inner() }
        .iter()
        .map(|m_enum| to_enum(m_enum));
    let intos = unsafe { enums.get_inner() }
    .iter()
    .map(|m_enum| to_intos(m_enum));
    let a = quote::quote! {
        const COUNT: usize = #len;
        #(#enum_streams)*
        #(#intos)*
    }
    .into();
    a
}
