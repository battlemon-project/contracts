use near_contract_standards::non_fungible_token::TokenId;
use slots_derive_macro::Slots;

#[test]
fn get_info_from_foo() {
    #[derive(Slots)]
    struct Foo {
        weapon_slot: Option<TokenId>,
        armor_slot: Option<TokenId>,
        _damage: u64,
    }

    let weapon_slot = Some("hello".to_string());
    let armor_slot = Some("world".to_string());
    let ret = Foo {
        weapon_slot,
        armor_slot,
        _damage: 0,
    };

    let actual = ret.slots_id();
    assert_eq!(actual, vec![&"hello".to_string(), &"world".to_string()]);
}

#[test]
fn get_info_from_foo_empty() {
    #[derive(Slots)]
    struct Foo {
        _damage: u64,
    }

    let ret = Foo { _damage: 0 };

    let actual = ret.slots_id();
    assert_eq!(actual, Vec::<&TokenId>::new());
}
