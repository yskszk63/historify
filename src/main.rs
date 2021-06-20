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

mod map_runes;
mod map_cuneiform;
mod map_hieroglyph;

fn map_ident(ident: &Ident, convert: fn(&str) -> String) -> Ident {
    let newname = convert(&ident.to_string());
    format_ident!("{}", newname)
}

struct V(fn(&str) -> String);

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

fn historify<R, W>(mut reader: R, mut writer: W, convert: fn(&str) -> String) -> io::Result<()> where R: Read, W: Write {
    let mut src = String::new();
    reader.read_to_string(&mut src).unwrap();

    let mut ast = syn::parse_file(&src).unwrap();
    V(convert).visit_file_mut(&mut ast);
    let prog = quote::quote! {
        #ast
    };

    write!(writer, "{}", prog)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let fun = map_hieroglyph::convert;
    if let Some(fname) =  env::args().nth(1) {
        let mut file = File::open(fname).unwrap();
        historify(&mut file, io::stdout(), fun)?;
    } else {
        historify(io::stdin(), io::stdout(), fun)?;
    }

    Ok(())
}
