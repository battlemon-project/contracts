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

            fn take_slot(&mut self, token_id: &TokenId) -> Option<TokenId> {
                let mut hashset = self.slots.take()?;
                let ret = hashset.take(token_id);
                if !hashset.is_empty() {
                    self.slots.get_or_insert(hashset);
                }
                ret
            }

            fn clear_slots(&mut self) -> Option<Vec<TokenId>> {
                self.slots.take().and_then(|v| Some(v.into_iter().collect()))
            }

            fn slots_id(&self) -> Option<Vec<TokenId>> {
                self.slots.clone().and_then(|v| Some(v.into_iter().collect()))
            }

           fn insert_slot(&mut self, token_id: &TokenId) -> bool {
                let mut slots = self.slots
                    .take()
                    .or_else(|| Some(HashSet::new()))
                    .unwrap();

                let ret = slots.insert(token_id.clone());
                self.slots = Some(slots);

                ret
           }

            fn replace_parent(&mut self, token_id: &TokenId) -> Option<TokenId> {
                self.parent.replace(token_id.clone())
            }
        }
    };

    gen.into()
}
