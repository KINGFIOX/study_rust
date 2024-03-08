extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ self, Data, Field };
use syn::DeriveInput;

/* ---------- ---------- hello ---------- ----------  */

#[proc_macro_derive(HelloMacro)]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap(); // abstract syntax tree
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen =
        quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("hello, macro! my name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

/* ---------- ---------- mydefault ---------- ----------  */

#[proc_macro_derive(MyDefault)]
pub fn my_default(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;

    let Data::Struct(s) = ast.data else { panic!("Mydefault derive macro must use in struct") };

    // 生命一个新的 ast，用于动态构建 字段赋值的 token
    let mut field_ast = quote!();

    for (idx, f) in s.fields.iter().enumerate() {
        let (field_id, field_ty) = (&f.ident, &f.ty);

        if field_id.is_none() {
            let field_idx = syn::Index::from(idx);
            field_ast.extend(
                quote! {
                #field_idx : #field_ty::default()
            }
            );
        } else {
            field_ast.extend(
                quote! {
                #(field_id.unwrap()) : #field_ty::default()
            }
            );
        }
    }

    (
        quote! {
        impl Default for #id {
            fn default() -> Self {
                Self {

                }
            }
        }
    }
    ).into()
}
