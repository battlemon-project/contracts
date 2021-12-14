use near_contract_standards::non_fungible_token::TokenId;
use parent_derive_macro::Parent;

trait Parent {
    fn take_parent(&mut self) -> Option<TokenId>;
}

#[test]
fn first() {
    #[derive(Parent)]
    struct Foo {
        parent: Option<TokenId>,
    }

    let token_id = "token_id".to_string();
    let mut foo = Foo {
        parent: Some(token_id.clone()),
    };

    assert_eq!(foo.take_parent(), Some(token_id));
    assert_eq!(foo.take_parent(), None);
    assert_eq!(foo.parent, None)
}
