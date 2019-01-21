#![recursion_limit = "192"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse;
use syn::Data::{Enum, Struct, Union};

#[proc_macro_derive(Padder)]
pub fn padder_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = parse(input).unwrap();
    let name = &ast.ident;

    match ast.data {
        Struct(s) => {
            let types = || s.fields.iter().map(|f| &f.ty);
            let sizes = types().map(|ty| quote!(std::mem::size_of::<#ty>()));
            let tys = types();
            let next_aligns = types()
                .map(|ty| quote!(<#ty>::alignof()))
                .skip(1)
                .chain(std::iter::once(quote!(#name::alignof())));
            let gen = quote! {
                impl Padder for #name {
                    fn padding_buf() -> &'static [u8] {
                      lazy_static::lazy_static! {
                        static ref BUF: [u8; std::mem::size_of::<#name>()] = {
                          let mut buf = [0xff; std::mem::size_of::<#name>()];
                          let mut offset = 0;
                          #({
                            let size = #sizes;
                            let next_offset = offset + size;
                            let next_align = #next_aligns;
                            let padding_size = next_offset % next_align;
                            let padding_size = (next_align - padding_size) % next_align;
                            let next_offset = next_offset + padding_size;
                            let mut count = 0;
                            for (i, b) in <#tys>::padding_buf().iter().enumerate() {
                              buf[offset + i] = *b;
                              count += 1;
                            }
                            for i in offset + count .. next_offset {
                              buf[i] = 0;
                            }
                            offset = next_offset;
                          })*
                          buf
                        };
                      }
                      &BUF[..]
                    }
                }
            };
            return gen.into();
        }
        Enum(_e) => unimplemented!(),
        Union(_u) => unimplemented!(),
    }
}

#[proc_macro_derive(Alignof)]
pub fn alignof_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = parse(input).unwrap();
    let name = &ast.ident;

    match ast.data {
        Struct(s) => {
            let types = || s.fields.iter().map(|f| &f.ty);
            let struct_align = types().map(|ty| quote!(<#ty>::alignof()));
            let gen = quote! {
                impl Alignof for #name {
                  fn alignof() -> usize {
                    [#(#struct_align),*].iter().max().unwrap().clone()
                  }
                }
            };
            return gen.into();
        }
        Enum(_) => unimplemented!(),
        Union(_) => {
            let gen = quote! {
              impl Alignof for #name {
                fn alignof() -> usize {
                  u8::alignof()
                }
              }
            };
            gen.into()
        }
    }
}
