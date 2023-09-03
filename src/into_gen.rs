use proc_macro2::{TokenStream, Ident, Span};

use crate::{MyEnum, Varient};

fn expand_varients(sub: &MyEnum) -> Vec<&Varient> {
    let mut exp: Vec<_> = sub.sub_enums.iter().map(
        |e| expand_varients(e)
    ).flatten().collect();
    for v in sub.varients.iter(){
        exp.push(v);
    }
    exp
}

fn expand_parens(n: usize) -> TokenStream {
    let it = (0..n).map(|n| {
        let a = Ident::new(&"a".repeat(n + 1), Span::call_site());
        quote::quote!{#a}
    });
    quote::quote!{
        ( #(#it,)* )
    }
}

fn sub_to_parent(sub: &MyEnum, parent: &MyEnum) -> TokenStream {
    let sub_name = &sub.name;
    let p_name = &parent.name;
    let exp = expand_varients(sub);
    let matches = exp.iter().map(|v| {
        let var_name = &v.id;
        if v.sub_vars.len() == 0 {
            quote::quote!{#sub_name::#var_name => #p_name::#var_name}
        }
        else {
            let par = expand_parens(v.sub_vars.len());
            quote::quote!{#sub_name::#var_name #par => #p_name::#var_name #par}
        }
    });
    quote::quote!{
        impl From<#sub_name> for #p_name {
            fn from(item: #sub_name) -> Self {
                match item {
                    #(#matches,)*
                }
            }
        }
    }
}

fn sub_expand(sub: &MyEnum, m_enum: &MyEnum) -> TokenStream {
    let sub_subs = sub.sub_enums.iter().map(|subsub| {
        sub_expand(subsub, m_enum)
    });
    let to = sub_to_parent(sub, m_enum);
    quote::quote!{
        #(#sub_subs)*
        #to
    }
}

pub(crate) fn to_intos(m_enum: &MyEnum) -> TokenStream {
    let sub_to_p = m_enum.sub_enums.iter().map(|sub| {
        sub_expand(sub, m_enum)
    });
    quote::quote!{
        #(#sub_to_p)*
    }
}