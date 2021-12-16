use near_contract_standards::non_fungible_token::TokenId;
use slots_derive_macro::Slots;
use test_utils::tokens;

/// Mocked version of original trait. It uses only for tests
trait Slots {
    fn slots_id(self) -> Vec<TokenId>;
    fn take_slots(&mut self) -> Vec<Option<TokenId>>;
}

#[test]
fn zero_slots_to_vec() {
    #[derive(Slots)]
    struct Model {
        _damage: u64,
    }

    let model = Model { _damage: 0 };

    let actual = model.slots_id();
    assert_eq!(actual, Vec::<TokenId>::new());
}

#[test]
fn one_slots_to_vec() {
    #[derive(Slots)]
    struct Model {
        weapon_slot: Option<TokenId>,
        _damage: u64,
    }

    let weapon = "bar".to_string();

    let model = Model {
        weapon_slot: Some(weapon.clone()),
        _damage: 0,
    };

    let actual = model.slots_id();
    assert_eq!(actual, vec![weapon])
}

#[test]
fn two_slots_to_vec() {
    #[derive(Slots)]
    struct Model {
        weapon_slot: Option<TokenId>,
        armor_slot: Option<TokenId>,
        _damage: u64,
    }
    let weapon = "bar".to_string();
    let armor = "baz".to_string();
    let weapon_slot = Some(weapon.clone());
    let armor_slot = Some(armor.clone());
    let model = Model {
        weapon_slot,
        armor_slot,
        _damage: 0,
    };

    let actual = model.slots_id();
    assert_eq!(actual, vec![weapon, armor]);
}

#[test]
fn take_slots() {
    #[derive(Slots)]
    struct Foo {
        parent: Option<TokenId>,
        a_slot: Option<TokenId>,
        b_slot: Option<TokenId>,
        c_slot: Option<TokenId>,
    }

    let [id1, id2, id3] = tokens::<3>();
    let mut foo = Foo {
        parent: None,
        a_slot: Some(id1.clone()),
        b_slot: Some(id2.clone()),
        c_slot: Some(id3.clone()),
    };

    assert_eq!(foo.take_slots(), vec![Some(id1), Some(id2), Some(id3)]);
    assert_eq!(foo.a_slot, None);
    assert_eq!(foo.b_slot, None);
    assert_eq!(foo.c_slot, None);
}
