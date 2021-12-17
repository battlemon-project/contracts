use manager_derive_macro::Manager;
use nft_models::manager::Manager;
use std::collections::HashSet;

type TokenId = String;

#[derive(Manager)]
struct Model {
    parent: Option<TokenId>,
    slots: Option<HashSet<TokenId>>,
}

#[test]
fn take_parent() {
    let id = "token_id".to_string();
    let mut model = Model {
        parent: Some(id.clone()),
        slots: None,
    };

    assert_eq!(model.take_parent(), Some(id));
    assert_eq!(model.take_parent(), None);
    assert_eq!(model.parent, None)
}

#[test]
fn take_slot() {
    let id = "1".to_string();
    let mut model = Model {
        parent: None,
        slots: Some([id.clone()].into()),
    };

    assert_eq!(model.take_slot(&"2".to_string()), None);
    assert_eq!(model.slots.clone().unwrap().len(), 1);
    assert_eq!(model.take_slot(&id), Some(id));
    assert_eq!(model.slots, None);
}

#[test]
fn clear_slots_with_none_nothing_change() {
    let parent_id = "2".to_string();
    let mut model = Model {
        parent: Some(parent_id.clone()),
        slots: None,
    };

    assert_eq!(model.clear_slots(), None);
    assert_eq!(model.parent, Some(parent_id));
}

#[test]
fn clear_slots_with_one_value() {
    let id = "1".to_string();
    let parent_id = "2".to_string();
    let mut model = Model {
        parent: Some(parent_id.clone()),
        slots: Some([id.clone()].into()),
    };

    assert_eq!(model.clear_slots(), Some(vec![id]));
    assert_eq!(model.parent, Some(parent_id));
    assert_eq!(model.slots, None);
}

#[test]
fn clear_slots_with_two_value() {
    let id1 = "1".to_string();
    let id2 = "2".to_string();
    let parent_id = "2".to_string();
    let mut model = Model {
        parent: Some(parent_id.clone()),
        slots: Some([id1.clone(), id2.clone()].into()),
    };
    let mut slots = model.clear_slots().unwrap();
    slots.sort();
    assert_eq!(slots, vec![id1, id2]);
    assert_eq!(model.parent, Some(parent_id));
    assert_eq!(model.slots, None);
}

#[test]
fn slots_id_with_empty() {
    let model = Model {
        parent: None,
        slots: None,
    };

    assert_eq!(model.slots_id(), None);
}

#[test]
fn slots_id_with_empty_slots() {
    let parent_id = "1".to_string();
    let model = Model {
        parent: Some(parent_id.clone()),
        slots: None,
    };

    assert_eq!(model.slots_id(), None);
    assert_eq!(model.parent, Some(parent_id));
}

#[test]
fn slots_id_with_one_slot() {
    let parent_id = "1".to_string();
    let id = "2".to_string();
    let model = Model {
        parent: Some(parent_id.clone()),
        slots: Some([id.clone()].into()),
    };

    assert_eq!(model.slots_id(), Some(vec![id.clone()]));
    assert_eq!(model.parent, Some(parent_id));
    assert_eq!(model.slots, Some([id].into()));
}

#[test]
fn slots_id_with_two_slots() {
    let parent_id = "1".to_string();
    let id1 = "2".to_string();
    let id2 = "3".to_string();
    let model = Model {
        parent: Some(parent_id.clone()),
        slots: Some([id1.clone(), id2.clone()].into()),
    };
    let mut slots = model.slots_id().unwrap();
    slots.sort();
    assert_eq!(slots, vec![id1.clone(), id2.clone()]);
    assert_eq!(model.parent, Some(parent_id));
    assert_eq!(model.slots, Some([id1, id2].into()));
}
