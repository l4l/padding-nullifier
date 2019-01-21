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
            let tys = types();
            let tys_ = types();
            let next_aligns1 = types()
                .map(|ty| quote!(<#ty>::alignof()))
                .skip(1)
                .chain(std::iter::once(quote!(#name::alignof())));
            let next_aligns2 = types()
                .map(|ty| quote!(<#ty>::alignof()))
                .skip(1)
                .chain(std::iter::once(quote!(#name::alignof())));
            let next_aligns3 = types()
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
                            let size = std::mem::size_of::<#tys>();
                            let next_offset = offset + size;
                            let padding_size = next_offset % #next_aligns1;
                            let padding_size = (#next_aligns2 - padding_size) % #next_aligns3;
                            let next_offset = next_offset + padding_size;
                            let mut count = 0;
                            for (i, b) in <#tys_>::padding_buf().iter().enumerate() {
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
