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
            let struct_align = types().map(|ty| quote!(std::mem::align_of::<#ty>()));
            let field_aligns = types().map(|ty| quote!(std::mem::align_of::<#ty>()));
            let field_sized = types().map(|ty| quote!(std::mem::size_of::<#ty>()));
            let gen = quote! {
                impl Padder for #name {
                    fn fill_padding(buf: &mut [u8]) {
                      let struct_align = [#(#struct_align),*].iter().max().unwrap();
                      let _ = [#(#field_sized),*]
                        .iter()
                        .zip([#(#field_aligns),*]
                          .iter()
                          .skip(1)
                          .chain(std::iter::once(struct_align)))
                        .fold(0, |offset, (field_size, next_align)| {
                          let field_end = offset + field_size;
                          let padding_size = next_align.checked_sub(*field_size).unwrap_or(0);
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
