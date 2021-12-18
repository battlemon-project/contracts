use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Manager)]
pub fn manager(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_manager(&ast)
}

fn impl_manager(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Manager for #name {
            fn take_parent(&mut self) -> Option<TokenId> {
                self.parent.take()
            }

            fn replace_parent(&mut self, token_id: &TokenId) -> Option<TokenId> {
                self.parent.replace(token_id.clone())
            }

            fn take_slot(&mut self, token_id: &TokenId) -> Option<TokenId> {
                self.slots.take(token_id)
            }

            fn drain_slots(&mut self) -> Vec<TokenId> {
                self.slots.drain().collect()
            }

            fn slots_id(&self) -> Vec<TokenId> {
                self.slots.clone().drain().collect()
            }

           fn insert_slot(&mut self, token_id: &TokenId) -> bool {
                self.slots.insert(token_id.clone())
           }
        }
    };

    gen.into()
}
