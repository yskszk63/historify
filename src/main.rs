use proc_macro2::TokenTree;
use syn::Macro;
use syn::ItemUse;
use syn::Visibility;
use syn::ItemMod;
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

struct V {
    use_stack: Vec<Ident>,
    use_vis_stack: Vec<Visibility>,
    need_rename: Vec<Vec<(Visibility, Ident)>>,
    convert: fn(&str) -> String,
}

impl V {
    fn new(convert: fn(&str) -> String) -> Self {
        Self {
            use_stack: vec![],
            use_vis_stack: vec![],
            need_rename: vec![],
            convert,
        }
    }
}

impl VisitMut for V {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        *i = map_ident(i, self.convert);
        visit_mut::visit_ident_mut(self, i);
    }

    fn visit_attribute_mut(&mut self, _: &mut Attribute) {
        // nop
    }

    fn visit_binding_mut(&mut self, i: &mut Binding) {
        visit_mut::visit_type_mut(self, &mut i.ty)
    }

    fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
        for it in &mut i.attrs {
            self.visit_attribute_mut(it);
        }
        self.visit_visibility_mut(&mut i.vis);
        // no ident changes
        // rename after
        self.need_rename.last_mut().unwrap().push((i.vis.clone(), i.ident.clone()));
        if let Some((_, items)) = &mut i.content {
            let mut newitems = vec![];
            for it in &mut *items {
                self.need_rename.push(vec![]);
                self.visit_item_mut(it);
                newitems.push(it.clone());
                for (visibility, need_rename) in self.need_rename.pop().unwrap() {
                    let rename = map_ident(&need_rename, self.convert);
                    let item = syn::parse2(quote! {
                        #[allow(unused_imports)]
                        #visibility use #need_rename as #rename;
                    }).unwrap();
                    newitems.push(item);
                }
            }
            *items = newitems;
        };
    }

    fn visit_use_tree_mut(&mut self, i: &mut UseTree) {
        match i {
            UseTree::Name(name) => {
                let ident = name.ident.clone();
                let rename = if ident == "self" && !self.use_stack.is_empty() {
                    map_ident(self.use_stack.last().unwrap(), self.convert)
                } else {
                    map_ident(&ident, self.convert)
                };
                *i = UseTree::Rename(UseRename {
                    ident,
                    as_token: Token![as](Span::call_site()),
                    rename,
                });
            }
            _ => visit_mut::visit_use_tree_mut(self, i)
        }
    }
    fn visit_use_rename_mut(&mut self, i: &mut UseRename) {
        if i.rename != "_" {
            self.need_rename.last_mut().unwrap().push((self.use_vis_stack.last().unwrap().clone(), i.rename.clone()));
        }
    }
    fn visit_use_path_mut(&mut self, i: &mut UsePath) {
        self.use_stack.push(i.ident.clone());
        self.visit_use_tree_mut(&mut i.tree);
        self.use_stack.pop();
    }

    fn visit_item_use_mut(&mut self, i: &mut ItemUse) {
        self.use_vis_stack.push(i.vis.clone());
        visit_mut::visit_item_use_mut(self, i);
        self.use_vis_stack.pop();
    }

    fn visit_signature_mut(&mut self, i: &mut Signature) {
        //https://github.com/rust-lang/rust/issues/28937
        if i.ident != "main" && !i.inputs.is_empty() {
            visit_mut::visit_signature_mut(self, i)
        }
    }

    fn visit_macro_mut(&mut self, i: &mut Macro) {
        self.visit_path_mut(&mut i.path);
        self.visit_macro_delimiter_mut(&mut i.delimiter);
        i.tokens = i.tokens.clone().into_iter().map(|t| match t {
            TokenTree::Ident(ident) => map_ident(&ident, self.convert).into(),
            _ => t,
        }).collect();
    }

    fn visit_file_mut(&mut self, i: &mut syn::File) {
        let attrs = syn::parse2::<syn::File>(quote! {
            //remove becuse cargo not work.
            //#![no_implicit_prelude]
            #![allow(uncommon_codepoints)]
        }).unwrap();
        for attr in attrs.attrs {
            i.attrs.push(attr.clone());
        }

        let prelude = syn::parse2(quote!{
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
                use std::option::Option::{self, Some, None};
                use std::result::Result::{self, Ok, Err};
                use std::string::{String, ToString};
                use std::vec::Vec;

            }
        }).unwrap();

        let useprelude = syn::parse2(quote!{
            #[allow(unused_imports)]
            use prelude::*;
        }).unwrap();

        let usemacro = syn::parse2(quote! {
            #[allow(unused_imports)]
            use std::{assert, assert_eq, assert_ne, cfg, column, compile_error, concat, dbg, debug_assert, debug_assert_eq, debug_assert_ne, env, eprint, eprintln, file, format, format_args, include, include_bytes, include_str, is_x86_feature_detected, line, matches, module_path, option_env, panic, print, println, stringify, thread_local, todo, unimplemented, unreachable, vec, write, writeln};
        }).unwrap();

        i.items.insert(0, usemacro);
        i.items.insert(0, useprelude);
        i.items.insert(0, prelude);

        for it in &mut i.attrs {
            self.visit_attribute_mut(it);
        }

        let mut newitems = vec![];
        for it in &mut *i.items {
            self.need_rename.push(vec![]);
            self.visit_item_mut(it);
            newitems.push(it.clone());
            for (visibility, need_rename) in self.need_rename.pop().unwrap() {
                let rename = map_ident(&need_rename, self.convert);
                let item = syn::parse2(quote! {
                    #[allow(unused_imports)]
                    #visibility use #need_rename as #rename;
                }).unwrap();
                newitems.push(item);
            }
        }
        i.items = newitems;

    }
}

fn historify<R, W>(mut reader: R, mut writer: W, convert: fn(&str) -> String) -> io::Result<()> where R: Read, W: Write {
    let mut src = String::new();
    reader.read_to_string(&mut src).unwrap();

    let mut ast = syn::parse_file(&src).unwrap();
    V::new(convert).visit_file_mut(&mut ast);
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
