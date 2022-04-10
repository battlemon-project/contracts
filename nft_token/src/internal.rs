use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::{AccountId, BorshStorageKey};

use token_metadata_ext::TokenExt;

use crate::error::Result;
use crate::Contract;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Metadata,
    NonFungibleToken,
    TokenMetadata,
    Enumeration,
    Approval,
    TokenModel,
}

impl Contract {
    pub(crate) fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let metadata = LazyOption::new(StorageKey::Metadata, Some(&metadata));
        let tokens = NonFungibleToken::new(
            StorageKey::NonFungibleToken,
            owner_id,
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );
        let model_by_id = LookupMap::new(StorageKey::TokenModel);

        Self {
            tokens,
            metadata,
            model_by_id,
            last_token_id: 0,
        }
    }

    // pub(crate) fn nested_tokens_id(
    //     &self,
    //     token_id: TokenId,
    //     buf: &mut Vec<(TokenId, ModelKind)>,
    // ) -> Result<()> {
    //     let model = self.model(&token_id)?;
    //     buf.push((token_id, model.clone()));
    //
    //     for id in model.slots_id() {
    //         self.nested_tokens_id(id, buf)?;
    //     }
    //
    //     Ok(())
    // }

    pub(crate) fn collect_ext_tokens(&self, tokens: Vec<Token>) -> Result<Vec<TokenExt>> {
        tokens
            .into_iter()
            .map(|token| {
                let model = self.model(&token.token_id)?;
                Ok(TokenExt::from_parts(token, model))
            })
            .collect()
    }

    // pub(crate) fn put_slot(&mut self, body_id: &TokenId, slot_id: &TokenId) -> Result<()> {
    // let mut body_model = self.model(&body_id)?;
    // let mut slot_model = self.model(&slot_id)?;
    // body_model.insert_slot(&slot_id);
    // slot_model.replace_parent(&body_id);
    // self.model_by_id.insert(&body_id, &body_model);
    // self.model_by_id.insert(&slot_id, &slot_model);
    //
    // Ok(())
    // }

    // pub(crate) fn check_instructions(&self, instructions: &[TokenId]) -> Result<()> {
    //     if instructions.is_empty() {
    //         return Err(BtlError::InstructionError(InstructionErrorKind::Empty));
    //     }
    //
    //     for chunk in instructions.chunks(2) {
    //         let body_id = chunk.get(0).ok_or(BtlError::InstructionError(
    //             InstructionErrorKind::ChunkBoundsOut,
    //         ))?;
    //         let slot_id = chunk.get(1).ok_or(BtlError::InstructionError(
    //             InstructionErrorKind::ChunkBoundsOut,
    //         ))?;
    //
    //         let body_owner = self.owner(body_id)?;
    //         let slot_owner = self.owner(slot_id)?;
    //
    //         if body_owner != slot_owner {
    //             return Err(BtlError::InstructionError(
    //                 InstructionErrorKind::NotEqualOwners,
    //             ));
    //         }
    //
    //         let body_model = self.model(body_id)?;
    //         let slot_model = self.model(slot_id)?;
    //         todo: add checking for amount attached models the same type (for example for Lemon is possible
    //         only attach two weapons)
    //         Like idea to store info about possible attachments like fields in models
    //         Lemon {
    //              ...,
    //              left_weapon: bool,
    //              right_weapon: bool,
    //              slots: [...]
    //         }
    //         also we can change slots type to Vec (now it's HashSet)
    //         and left_weapon, right_weapon to Option<usize>, for storing indices of tokens id's
    //         benefits: can see used slots, understand id's of this slots, low memory usage, fast operations.
    //         todo: added additional tests for these cases
    //
    //         if !body_model.is_compatible(&slot_model) {
    //             return Err(BtlError::InstructionError(
    //                 InstructionErrorKind::IncompatibleModels,
    //             ));
    //         }
    //     }
    //
    //     Ok(())
    // }

    // pub(crate) fn disassemble_token(&mut self, token_id: &TokenId) -> Result<()> {
    //     // TODO: to cover with unittests
    //     let mut model = self.model(&token_id)?;
    //
    //     if let Some(parent_id) = model.take_parent() {
    //         let mut parent = self.model(&parent_id)?;
    //         parent.take_slot(&token_id);
    //         self.model_by_id.insert(&parent_id, &parent);
    //     };
    //
    //     let slots = model.drain_slots();
    //     self.model_by_id.insert(&token_id, &model);
    //
    //     for id in slots {
    //         let mut child = self.model(&id)?;
    //         child.take_parent();
    //         self.model_by_id.insert(&id, &child);
    //     }
    //
    //     Ok(())
    // }
}

// #[cfg(all(test, not(target_arch = "wasm32")))]
// mod tests {
//     use std::collections::HashSet;
//
//     use near_sdk::{serde_json, testing_env};
//
//     use nft_models::lemon::Lemon;
//     use nft_models::suppressor::Suppressor;
//     use nft_models::weapon::{Type as WeaponType, Weapon};
//     use test_utils::*;
//     use token_metadata_ext::TokenMetadataExt;
//
//     use super::*;
//
//     const MINT_STORAGE_COST: u128 = 6_000_000_000_000_000_000_000;
//
//     #[test]
//     #[ignore]
//     fn token_metadata_deserialize() {
//         let json = r#"{"title": "Title for token 1", "description": "some description for batllemon nft token", "media": "blabla", "properties": {"option": "on_sale", "century": "our_time", "type": "light", "lemon_gen": "nakamoto", "background": "red", "top": "headdress", "cyber_suit": "metallic", "expression": "brooding", "eyes": "open", "hair": "bob_marley", "accessory": "cigar", "winrate": 14, "rarity": 12}}"#;
//         let _token_metadata: TokenMetadataExt = serde_json::from_str(json).unwrap();
//     }
//
//     #[test]
//     #[should_panic = "value: couldn't find the model for token with id: 0"]
//     fn nested_tokens_id() {
//         let contract = Contract::init(alice());
//         let [token_id] = tokens::<1>();
//         contract.nested_tokens_id(token_id, &mut vec![]).unwrap();
//     }
//
//     #[test]
//     fn nested_tokens_id_must_return_self() {
//         let mut contract = Contract::init(alice());
//         let lemon = get_foo_lemon();
//         let [token_id] = tokens::<1>();
//         contract.model_by_id.insert(&token_id, &lemon.into());
//
//         let mut buf = Vec::new();
//         contract
//             .nested_tokens_id(token_id.clone(), &mut buf)
//             .unwrap();
//         assert_eq!(buf.len(), 1);
//         assert_eq!(buf[0].0, *token_id);
//     }
//
//     #[test]
//     fn nested_tokens_id_must_return_self_and_weapon() {
//         let mut contract = Contract::init(alice());
//         let weapon = get_foo_weapon().into();
//
//         let [weapon_token_id, lemon_token_id] = tokens::<2>();
//         contract.model_by_id.insert(&weapon_token_id, &weapon);
//
//         let lemon = Lemon {
//             slots: [weapon_token_id.clone()].into(),
//             ..get_foo_lemon()
//         }
//         .into();
//         contract.model_by_id.insert(&lemon_token_id, &lemon);
//
//         let mut weapon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(weapon_token_id.clone(), &mut weapon_nested_buf)
//             .unwrap();
//         assert_eq!(
//             weapon_nested_buf,
//             vec![(weapon_token_id.clone(), weapon.clone())]
//         );
//
//         let mut lemon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(lemon_token_id.clone(), &mut lemon_nested_buf)
//             .unwrap();
//         assert_eq!(
//             lemon_nested_buf,
//             vec![(lemon_token_id, lemon), (weapon_token_id, weapon)]
//         );
//     }
//
//     #[test]
//     fn nested_tokens_must_return_self_and_two_weapons() {
//         let mut contract = Contract::init(alice());
//
//         let left_weapon = get_foo_weapon();
//         let right_weapon = Weapon {
//             level: 1,
//             r#type: WeaponType::Collusion,
//             ..left_weapon.clone()
//         };
//
//         let [left_weapon_token_id, right_weapon_token_id, lemon_token_id] = tokens::<3>();
//
//         let lemon_slots = [left_weapon_token_id.clone(), right_weapon_token_id.clone()].into();
//         let lemon = Lemon {
//             slots: lemon_slots,
//             ..get_foo_lemon()
//         };
//
//         contract.model_by_id.extend([
//             (left_weapon_token_id.clone(), left_weapon.clone().into()),
//             (right_weapon_token_id.clone(), right_weapon.clone().into()),
//             (lemon_token_id.clone(), lemon.clone().into()),
//         ]);
//
//         let mut left_weapon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(left_weapon_token_id.clone(), &mut left_weapon_nested_buf)
//             .unwrap();
//         assert_eq!(
//             left_weapon_nested_buf,
//             vec![(left_weapon_token_id.clone(), left_weapon.clone().into())]
//         );
//
//         let mut right_weapon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(right_weapon_token_id.clone(), &mut right_weapon_nested_buf)
//             .unwrap();
//         assert_eq!(
//             right_weapon_nested_buf,
//             vec![(right_weapon_token_id.clone(), right_weapon.clone().into())]
//         );
//
//         let mut lemon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(lemon_token_id.clone(), &mut lemon_nested_buf)
//             .unwrap();
//         lemon_nested_buf.sort_by_key(|k| k.0.clone());
//         assert_eq!(
//             lemon_nested_buf,
//             vec![
//                 (left_weapon_token_id, left_weapon.into()),
//                 (right_weapon_token_id, right_weapon.into()),
//                 (lemon_token_id, lemon.into())
//             ]
//         );
//     }
//
//     #[test]
//     fn nested_tokens_must_return_self_and_two_weapons_and_right_weapon_suppressor() {
//         let mut contract = Contract::init(alice());
//
//         let [left_weapon_token_id, right_weapon_token_id, lemon_token_id, suppressor_token_id] =
//             tokens::<4>();
//
//         let suppressor = Suppressor {
//             parent: None,
//             slots: HashSet::new(),
//         };
//
//         let left_weapon = get_foo_weapon();
//         let right_weapon = Weapon {
//             level: 1,
//             r#type: WeaponType::Projection,
//             slots: [suppressor_token_id.clone()].into(),
//             ..left_weapon.clone()
//         };
//
//         let lemon_slots = [left_weapon_token_id.clone(), right_weapon_token_id.clone()].into();
//         let lemon = Lemon {
//             slots: lemon_slots,
//             ..get_foo_lemon()
//         };
//
//         contract.model_by_id.extend([
//             (left_weapon_token_id.clone(), left_weapon.clone().into()),
//             (right_weapon_token_id.clone(), right_weapon.clone().into()),
//             (suppressor_token_id.clone(), suppressor.clone().into()),
//             (lemon_token_id.clone(), lemon.clone().into()),
//         ]);
//
//         let mut left_weapon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(left_weapon_token_id.clone(), &mut left_weapon_nested_buf)
//             .unwrap();
//         assert_eq!(
//             left_weapon_nested_buf,
//             vec![(left_weapon_token_id.clone(), left_weapon.clone().into())]
//         );
//
//         let mut right_weapon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(right_weapon_token_id.clone(), &mut right_weapon_nested_buf)
//             .unwrap();
//
//         let key = |k: &(TokenId, ModelKind)| k.0.clone();
//         right_weapon_nested_buf.sort_by_key(key);
//         assert_eq!(
//             right_weapon_nested_buf,
//             vec![
//                 (right_weapon_token_id.clone(), right_weapon.clone().into()),
//                 (suppressor_token_id.clone(), suppressor.clone().into())
//             ]
//         );
//
//         let mut lemon_nested_buf = Vec::new();
//         contract
//             .nested_tokens_id(lemon_token_id.clone(), &mut lemon_nested_buf)
//             .unwrap();
//         lemon_nested_buf.sort_by_key(key);
//         assert_eq!(
//             lemon_nested_buf,
//             vec![
//                 (left_weapon_token_id, left_weapon.into()),
//                 (right_weapon_token_id, right_weapon.into()),
//                 (lemon_token_id, lemon.into()),
//                 (suppressor_token_id, suppressor.into()),
//             ]
//         );
//     }
//
//     #[test]
//     #[should_panic(expected = "value: couldn't find the model for token with id: 0")]
//     fn put_slot_body_do_not_exist() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let weapon_meta = fake_metadata_with(get_foo_weapon());
//         contract.mint(weapon_id.clone(), weapon_meta, Some(bob()));
//         contract.put_slot(&lemon_id, &weapon_id).unwrap();
//     }
//
//     #[test]
//     #[should_panic(expected = "value: couldn't find the model for token with id: 1")]
//     fn put_slot_when_slot_do_not_exist() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         contract.mint(lemon_id.clone(), lemon_meta, Some(bob()));
//         contract.put_slot(&lemon_id, &weapon_id).unwrap();
//     }
//
//     #[test]
//     fn put_slot_when_body_and_slot_is_compatible() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST * 2).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         let weapon_meta = fake_metadata_with(get_foo_weapon());
//         contract.mint(lemon_id.clone(), lemon_meta, Some(bob()));
//         contract.mint(weapon_id.clone(), weapon_meta, Some(bob()));
//         contract.put_slot(&lemon_id, &weapon_id).unwrap();
//     }
//
//     #[test]
//     #[should_panic(expected = "value: couldn't find owner with id: 0")]
//     fn check_instructions_body_do_not_exist() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let weapon_meta = fake_metadata_with(get_foo_weapon());
//         contract.mint(weapon_id.clone(), weapon_meta, Some(bob()));
//         contract.check_instructions(&[lemon_id, weapon_id]).unwrap();
//     }
//
//     #[test]
//     #[should_panic(expected = "value: couldn't find owner with id: 1")]
//     fn check_instructions_when_slot_do_not_exist() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         contract.mint(lemon_id.clone(), lemon_meta, Some(bob()));
//         contract.check_instructions(&[lemon_id, weapon_id]).unwrap();
//     }
//
//     #[test]
//     #[should_panic(
//         expected = "value: provided instructions contain errors because: owners for token's models must be the same"
//     )]
//     fn check_instructions_when_body_and_slot_not_the_same_owner() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST * 2).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         let weapon_meta = fake_metadata_with(get_foo_weapon());
//         contract.mint(lemon_id.clone(), lemon_meta, Some(bob()));
//         contract.mint(weapon_id.clone(), weapon_meta, Some(carol()));
//         contract.check_instructions(&[lemon_id, weapon_id]).unwrap();
//     }
//
//     #[test]
//     #[should_panic(
//         expected = "value: provided instructions contain errors because: models are not compatible"
//     )]
//     fn check_instructions_when_body_and_slot_is_not_compatible() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST * 2).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, suppressor_id] = tokens::<2>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         let suppressor_meta = fake_metadata_with(Suppressor {
//             parent: None,
//             slots: HashSet::new(),
//         });
//         contract.mint(lemon_id.clone(), lemon_meta, Some(bob()));
//         contract.mint(suppressor_id.clone(), suppressor_meta, Some(bob()));
//         contract
//             .check_instructions(&[lemon_id, suppressor_id])
//             .unwrap();
//     }
//
//     #[test]
//     fn check_instructions_when_body_and_slot_is_compatible() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST * 2).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         let weapon_meta = fake_metadata_with(get_foo_weapon());
//         contract.mint(lemon_id.clone(), lemon_meta, Some(bob()));
//         contract.mint(weapon_id.clone(), weapon_meta, Some(bob()));
//         contract.check_instructions(&[lemon_id, weapon_id]).unwrap();
//     }
//
//     #[test]
//     #[should_panic(
//         expected = " value: provided instructions contain errors because: empty instructions are not allowed"
//     )]
//     fn check_instructions_when_empty_instructions() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST * 2).build());
//         let mut contract = Contract::init(alice());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         let weapon_meta = fake_metadata_with(get_foo_weapon());
//         contract.mint(lemon_id.clone(), lemon_meta, Some(bob()));
//         contract.mint(weapon_id.clone(), weapon_meta, Some(bob()));
//         contract.check_instructions(&[]).unwrap();
//     }
//
//     #[test]
//     #[should_panic(
//         expected = "value: provided instructions contain errors because: index out of bound in chunk"
//     )]
//     fn check_instructions_when_not_enough_items_for_chunk() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(MINT_STORAGE_COST * 2).build());
//         let contract = Contract::init(alice());
//
//         let [lemon_id] = tokens::<1>();
//         let lemon_meta = fake_metadata_with(get_foo_lemon());
//         contract.check_instructions(&[lemon_id]).unwrap();
//     }
// }
