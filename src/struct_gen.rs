use proc_macro2::TokenStream;

use crate::MyEnum;

fn to_enum_inner(inner: &MyEnum) -> TokenStream {
    let sub_varients = inner.sub_enums.iter().map(|en| to_enum_inner(en));
    let varients = inner.varients.iter().map(|varient| {
        let v = &varient.id;
        let sub_vars = &varient.sub_vars;
        if sub_vars.len() == 0 {
            quote::quote! {#v,}
        } else {
            let sub_vars = sub_vars.iter().map(|v| {
                let id = &v.name;
                quote::quote! {#id,}
            });
            quote::quote! {#v ( #(#sub_vars)* ),}
        }
    });
    quote::quote! {
        #(#sub_varients)*
        #(#varients)*
    }
}

pub(crate) fn to_enum(m_enum: &MyEnum) -> TokenStream {
    let varients = to_enum_inner(m_enum);
    let name = &m_enum.name;
    quote::quote! {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
        pub enum #name {
            #varients
        }
    }
}
