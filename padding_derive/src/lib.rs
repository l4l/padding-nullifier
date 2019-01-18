#![recursion_limit = "128"]
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
            let field_aligns = types().map(|ty| quote!(<#ty>::alignof()));
            let next_aligns = types().map(|ty| quote!(<#ty>::alignof()));
            let field_sizes = types().map(|ty| quote!(std::mem::size_of::<#ty>()));
            let gen = quote! {
                impl Padder for #name {
                    fn fill_padding(buf: &mut [u8]) {
                      let struct_align = #name::alignof();
                      let _ = [#(#field_sizes),*]
                        .iter()
                        .zip([#(#next_aligns),*]
                          .iter()
                          .skip(1)
                          .chain(std::iter::once(&struct_align)))
                        .zip([#(#field_aligns),*].iter())
                        .fold(0, |offset, ((field_size, next_align), field_align)| {
                          let field_end = offset + field_size;
                          let padding_size = field_end % next_align;
                          let padding_size = (next_align - padding_size) % next_align;
                          let next_offset = field_end + padding_size;

                          for mut b in buf[field_end..next_offset].iter_mut() {
                            *b = 0;
                          }
                          next_offset
                        });
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
