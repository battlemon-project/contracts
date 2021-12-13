use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

/// derive macro for `Slots` trait, collect fields with `slot` in the name into `Vec<&TokenId>` of ref on values.
#[proc_macro_derive(Slots)]
pub fn slots(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_slots(&ast)
}

fn impl_slots(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let data_struct = match &ast.data {
        Data::Struct(data) => data,
        _ => panic!("works only with struct"),
    };

    let fields = match &data_struct.fields {
        Fields::Named(FieldsNamed { named, .. }) => named.iter().flat_map(|v| &v.ident),
        _ => panic!("works only with named fields"),
    };

    let slots = fields.filter(|v| v.to_string().contains("slot"));

    let gen = quote! {
        impl Slots for #name {
            fn slots_id(self) -> Vec<TokenId> {
                std::iter::empty()
                    #(.chain(self.#slots))*
                    .collect()
            }
        }
    };
    gen.into()
}
