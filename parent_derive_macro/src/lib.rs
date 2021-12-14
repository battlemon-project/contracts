use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

/// derive macro for `Parent` trait, take field `parent`.
#[proc_macro_derive(Parent)]
pub fn parent(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_parent(&ast)
}

fn impl_parent(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Parent for #name {
            fn take_parent(&mut self) -> Option<TokenId> {
                self.parent.take()
            }
        }
    };

    gen.into()
}
