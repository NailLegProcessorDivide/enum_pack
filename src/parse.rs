use std::iter::Peekable;

use proc_macro2::{Ident, TokenStream, TokenTree};
use proc_macro_error::abort;

use crate::{wo_vec::WoVec, MyEnum, Varient};

fn find_in_enums(enums: &WoVec<MyEnum>, id: Ident) -> Option<&'static MyEnum> {
    for i in 0..enums.len() {
        let sub_enum = unsafe { enums.get(i) };
        if sub_enum.name == id {
            return Some(sub_enum);
        }
    }
    None
}

fn consume_seperator_char(
    m_tokens: &mut Peekable<<TokenStream as IntoIterator>::IntoIter>,
    c: char,
) {
    match m_tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == c => {}
        None => {}
        tok => abort!(tok, "expected {c} found {:?}", tok),
    }
}

fn parse_enum_sub_varients(
    m_tokens: &mut Peekable<<TokenStream as IntoIterator>::IntoIter>,
    enums: &WoVec<MyEnum>,
) -> Vec<&'static MyEnum> {
    let mut vars = Vec::new();
    while let Some(tok) = m_tokens.next() {
        let id = match tok {
            TokenTree::Ident(id) => id,
            tok => abort!(tok, "expected inentifier, found {}", tok),
        };
        consume_seperator_char(m_tokens, ',');
        let en = match find_in_enums(enums, id.clone()) {
            Some(en) => en,
            None => abort!(id, "id needs to be defined before use {}", id),
        };
        vars.push(en);
    }
    vars
}

fn parse_enum_inner(
    m_tokens: &mut Peekable<<TokenStream as IntoIterator>::IntoIter>,
    enums: &WoVec<MyEnum>,
) -> Vec<Varient> {
    let mut varients = Vec::new();
    while m_tokens.peek().is_some() {
        let id = match m_tokens.next() {
            Some(TokenTree::Ident(id)) => id,
            t => abort!(t, "expected Ident"),
        };

        let sub_vars = if let Some(TokenTree::Group(g)) = m_tokens.peek() {
            let g = g.clone();
            m_tokens.next();
            let mut stream = g.stream().into_iter().peekable();
            parse_enum_sub_varients(&mut stream, enums)
        } else {
            Vec::new()
        };

        consume_seperator_char(m_tokens, ',');
        varients.push(Varient {
            id: id.clone(),
            sub_vars,
        });
    }
    varients
}

pub(crate) fn parse_enum(
    m_tokens: &mut Peekable<<TokenStream as IntoIterator>::IntoIter>,
    enums: &WoVec<MyEnum>,
) -> MyEnum {
    let name = match m_tokens.next() {
        Some(TokenTree::Ident(v)) => v,
        id => panic!("{:#?} not Identifyer", id),
    };

    match m_tokens.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '=' => {}
        eq => {
            abort!(eq, "expected '=' found {:?}", eq);
        }
    }

    let mut sub_enums = Vec::new();

    while let Some(TokenTree::Ident(id)) = m_tokens.peek() {
        // clone to not borrow during next reads
        let id = id.clone();
        //clear read token
        m_tokens.next();
        // parse '+'token
        match m_tokens.next() {
            Some(TokenTree::Punct(c)) if c.as_char() == '+' => {}
            Some(tok) => abort!(tok, "expected '+' found {}", tok),
            None => abort!(id, "expected '+' following"),
        }
        // TODO: switch to iterator for enums
        let mut found = false;
        for i in 0..enums.len() {
            let sub_enum = unsafe { enums.get(i) };
            if sub_enum.name == id {
                sub_enums.push(sub_enum);
                found = true;
                break;
            }
        }
        if !found {
            abort!(id, "enum id not previously defined: {}", id)
        }
    }

    let mut sub_enum = Vec::new();
    let toks = m_tokens.next();
    match toks {
        Some(TokenTree::Group(g)) => {
            let mut stream = g.stream().into_iter().peekable();
            sub_enum.append(&mut parse_enum_inner(&mut stream, enums))
        }
        _ => {
            abort!(toks, "expected group found {:?}", toks);
        }
    }

    MyEnum {
        name,
        sub_enums,
        varients: sub_enum,
    }
}
