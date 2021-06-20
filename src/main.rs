use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

use syn::Attribute;
use syn::Signature;
use syn::Binding;
use syn::UseRename;
use syn::{UsePath, UseTree};

use proc_macro2::Span;
use syn::{Ident, Token};
use syn::visit_mut::{self, VisitMut};
use quote::{format_ident, quote};

fn map_runes(c: char) -> &'static str {
    match c {
        'A' => "\u{16A0}",
        'B' => "\u{16A1}",
        'C' => "\u{16A2}",
        'D' => "\u{16A3}",
        'E' => "\u{16A4}",
        'F' => "\u{16A5}",
        'G' => "\u{16A6}",
        'H' => "\u{16A7}",
        'I' => "\u{16A8}",
        'J' => "\u{16A9}",
        'K' => "\u{16AA}",
        'L' => "\u{16AB}",
        'M' => "\u{16AC}",
        'N' => "\u{16AD}",
        'O' => "\u{16AE}",
        'P' => "\u{16AF}",
        'Q' => "\u{16B0}",
        'R' => "\u{16B1}",
        'S' => "\u{16B2}",
        'T' => "\u{16B3}",
        'U' => "\u{16B4}",
        'V' => "\u{16B5}",
        'W' => "\u{16B6}",
        'X' => "\u{16B7}",
        'Y' => "\u{16B8}",
        'Z' => "\u{16B9}",
        'a' => "\u{16BA}",
        'b' => "\u{16BB}",
        'c' => "\u{16BC}",
        'd' => "\u{16BD}",
        'e' => "\u{16BE}",
        'f' => "\u{16BF}",
        'g' => "\u{16C0}",
        'h' => "\u{16C1}",
        'i' => "\u{16C2}",
        'j' => "\u{16C3}",
        'k' => "\u{16C4}",
        'l' => "\u{16C0}",
        'm' => "\u{16C0}",
        'n' => "\u{16C0}",
        'o' => "\u{16C5}",
        'p' => "\u{16C6}",
        'q' => "\u{16C7}",
        'r' => "\u{16C8}",
        's' => "\u{16C9}",
        't' => "\u{16CA}",
        'u' => "\u{16CB}",
        'v' => "\u{16CC}",
        'w' => "\u{16CD}",
        'x' => "\u{16CE}",
        'y' => "\u{16CF}",
        'z' => "\u{16D0}",
        '0' => "\u{16D1}",
        '1' => "\u{16D2}",
        '2' => "\u{16D3}",
        '3' => "\u{16D4}",
        '4' => "\u{16D5}",
        '5' => "\u{16D6}",
        '6' => "\u{16D7}",
        '7' => "\u{16D8}",
        '8' => "\u{16D9}",
        '9' => "\u{16DA}",
        '+' => "\u{16DB}",
        '/' => "\u{16DC}",
        '=' => "\u{16DD}",
        c => panic!("{}", c),
    }
}

fn map_ident(ident: &Ident, map: fn(char) -> &'static str) -> Ident {
    let newname = base64::encode(ident.to_string().as_bytes());
    //let newname = format!("{}_foo", ident);
    let newname = newname.chars().map(map).collect::<String>();
    format_ident!("{}", newname)
}

struct V(fn(char) -> &'static str);

impl VisitMut for V {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        *i = map_ident(i, self.0);
        visit_mut::visit_ident_mut(self, i);
    }

    fn visit_attribute_mut(&mut self, _: &mut Attribute) {
        // nop
    }

    fn visit_binding_mut(&mut self, i: &mut Binding) {
        visit_mut::visit_type_mut(self, &mut i.ty)
    }

    fn visit_use_tree_mut(&mut self, i: &mut UseTree) {
        match i {
            UseTree::Name(name) => {
                let ident = name.ident.clone();
                let rename = map_ident(&ident, self.0);
                *i = UseTree::Rename(UseRename {
                    ident,
                    as_token: Token![as](Span::call_site()),
                    rename,
                });
            }
            _ => visit_mut::visit_use_tree_mut(self, i)
        }
    }
    fn visit_use_path_mut(&mut self, i: &mut UsePath) {
        self.visit_use_tree_mut(&mut i.tree)
    }

    fn visit_signature_mut(&mut self, i: &mut Signature) {
        if i.ident != "main" && !i.inputs.is_empty() {
            visit_mut::visit_signature_mut(self, i)
        }
    }

    fn visit_file_mut(&mut self, i: &mut syn::File) {
        visit_mut::visit_file_mut(self, i);

        let attrs = syn::parse2::<syn::File>(quote! {
            //remove becuse cargo not work.
            //#![no_implicit_prelude]
            #![allow(uncommon_codepoints)]
        }).unwrap();
        for attr in attrs.attrs {
            i.attrs.push(attr.clone());
        }

        let mut prelude = syn::parse2(quote!{
            mod prelude {
                #![allow(unused_imports)]
                use std::marker::{Copy, Send, Sized, Sync, Unpin};
                use std::ops::{Drop, Fn, FnMut, FnOnce};
                use std::mem::drop;
                use std::boxed::Box;
                use std::borrow::ToOwned;
                use std::clone::Clone;
                use std::cmp::{PartialEq, PartialOrd, Eq, Ord};
                use std::convert::{AsRef, AsMut, Into, From};
                use std::default::Default;
                use std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator};
                //use std::option::Option::{self, Some, None};
                use std::option::Option;
                use std::option::Option::{Some, None};
                //use std::result::Result::{self, Ok, Err};
                use std::result::Result;
                use std::result::Result::{Ok, Err};
                use std::string::{String, ToString};
                use std::vec::Vec;

            }
        }).unwrap();
        self.visit_item_mut(&mut prelude);

        let mut ident = format_ident!("prelude");
        self.visit_ident_mut(&mut ident);
        let mut useprelude = syn::parse2(quote!{
            #[allow(unused_imports)]
            use #ident::*;
        }).unwrap();
        self.visit_item_mut(&mut useprelude);

        let mut usemacro = syn::parse2(quote! {
            #[allow(unused_imports)]
            use std::{assert, assert_eq, assert_ne, cfg, column, compile_error, concat, dbg, debug_assert, debug_assert_eq, debug_assert_ne, env, eprint, eprintln, file, format, format_args, include, include_bytes, include_str, is_x86_feature_detected, line, matches, module_path, option_env, panic, print, println, stringify, thread_local, todo, unimplemented, unreachable, vec, write, writeln};
        }).unwrap();
        self.visit_item_mut(&mut usemacro);

        i.items.insert(0, prelude);
        i.items.insert(1, useprelude);
        i.items.insert(2, usemacro);
    }
}

fn historify<R, W>(mut reader: R, mut writer: W, map: fn(char) -> &'static str) -> io::Result<()> where R: Read, W: Write {
    let mut src = String::new();
    reader.read_to_string(&mut src).unwrap();

    let mut ast = syn::parse_file(&src).unwrap();
    V(map).visit_file_mut(&mut ast);
    let prog = quote::quote! {
        #ast
    };

    write!(writer, "{}", prog)?;
    Ok(())
}

fn main() -> io::Result<()> {
    if let Some(fname) =  env::args().nth(1) {
        let mut file = File::open(fname).unwrap();
        historify(&mut file, io::stdout(), map_runes)?;
    } else {
        historify(io::stdin(), io::stdout(), map_runes)?;
    }

    Ok(())
}
