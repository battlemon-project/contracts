use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::env::panic_str;
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, Promise};

use consts::DATA_IMAGE_SVG_LEMON_LOGO;
use nft_models::ModelKind;
use token_metadata_ext::{TokenExt, TokenMetadataExt};

mod consts;
mod error;
mod helpers;
mod internal;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    model_by_id: LookupMap<TokenId, ModelKind>,
    last_token_id: u128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Battlemon".to_string(),
            symbol: "BTLMN".to_string(),
            icon: Some(DATA_IMAGE_SVG_LEMON_LOGO.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();

        Self::new(owner_id, metadata)
    }

    #[payable]
    pub fn nft_mint(&mut self, receiver_id: AccountId) -> TokenExt {
        use nft_models::Lemon;

        let owner_id = receiver_id;
        self.last_token_id += 1;

        let random = helpers::get_random_arr_range(0, 100);

        let model = ModelKind::Lemon(Lemon::from_random(&random));

        let token_metadata_ext = TokenMetadataExt {
            title: None,
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
            model: model.clone(),
        };

        let (mut metadata, _) = token_metadata_ext.split();
        // for test purpose
        metadata.media = Some(
            "https://api.monosnap.com/file/download?id=axPubUzmo1iTBzOr1yn7PhauYfvL8r".to_string(),
        );
        let token_id = self.last_token_id.to_string();
        self.model_by_id.insert(&token_id, &model);
        let token = self
            .tokens
            .internal_mint(token_id.clone(), owner_id, Some(metadata));
        TokenExt::from_parts(token, model)
    }

    pub fn get_owner_by_token_id(&self, token_id: TokenId) -> Option<AccountId> {
        self.tokens.owner_by_id.get(&token_id)
    }

    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<TokenExt> {
        let tokens = self.tokens.nft_tokens(from_index, limit);

        self.collect_ext_tokens(tokens)
            .expect("Couldn't collect tokens in extended format.")
    }

    pub fn nft_total_supply(&self) -> U128 {
        self.tokens.nft_total_supply()
    }

    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        self.tokens.nft_supply_for_owner(account_id)
    }

    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<TokenExt> {
        let tokens = self
            .tokens
            .nft_tokens_for_owner(account_id, from_index, limit);

        self.collect_ext_tokens(tokens)
            .expect("Couldn't collect tokens in extended format.")
    }

    #[payable]
    pub fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        self.tokens
            .nft_transfer(receiver_id, token_id, approval_id, memo);

        // After transfer NFT, we must disassemble compound NFT.
        // Put off token with `token_id` from tokens that contain it.
        // self.disassemble_token(&token_id)
        //     .expect("Couldn't disassemble token");
    }

    // todo: add security checking
    #[payable]
    pub fn update_token_media(&mut self, token_id: TokenId, new_media: String) {
        let token_metadata = self
            .nft_token(token_id.clone())
            .and_then(|token| token.metadata)
            .map(|metadata| TokenMetadata {
                media: Some(new_media),
                ..metadata
            })
            .unwrap_or_else(|| panic_str("metadata for token doesn't exists"));

        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &token_metadata));
    }
    // #[payable]
    // pub fn nft_transfer_call(
    //     &mut self,
    //     receiver_id: AccountId,
    //     token_id: TokenId,
    //     approval_id: Option<u64>,
    //     memo: Option<String>,
    //     msg: String,
    // ) -> PromiseOrValue<bool> {
    //     self.tokens
    //         .nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
    // }

    pub fn nft_token(&self, token_id: TokenId) -> Option<TokenExt> {
        self.tokens.nft_token(token_id).map(|token| {
            let model = self
                .model(&token.token_id)
                .expect("Couldn't provide nft token");

            TokenExt::from_parts(token, model)
        })
    }

    // #[payable]
    // pub fn assemble_compound_nft(&mut self, instructions: Vec<TokenId>) {
    //     assert_one_yocto();
    //     self.check_instructions(&instructions)
    //         .expect("Provided instructions contain errors");
    //
    //     for chunks in instructions.as_slice().chunks(2) {
    //         self.put_slot(&chunks[0], &chunks[1])
    //             .expect("Couldn't assemble compound nft");
    //     }
    // }

    // pub fn compound_nft_token(&self, token_id: TokenId) -> Vec<(TokenId, ModelKind)> {
    //     //todo: add tests
    //     let mut buf = Vec::new();
    //     self.nested_tokens_id(token_id, &mut buf)
    //         .expect("Couldn't get nested tokens");
    //     buf
    // }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().expect("Metadata didn't set")
    }
}

near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);

// #[cfg(all(test, not(target_arch = "wasm32")))]
// mod tests {
//     use std::collections::{HashMap, HashSet};
//
//     use near_sdk::testing_env;
//
//     use nft_models::lemon::Lemon;
//     use nft_models::suppressor::Suppressor;
//     use nft_models::weapon::Weapon;
//     use test_utils::*;
//
//     use super::*;
//
//     const MINT_STORAGE_COST: u128 = 6_000_000_000_000_000_000_000;
//
//     #[test]
//     fn init() {
//         let mut context = get_context(bob());
//         testing_env!(context.build());
//         let contract = Contract::init(bob());
//         testing_env!(context.is_view(true).build());
//         assert_eq!(contract.nft_total_supply().0, 0);
//     }
// }

// #[cfg(all(test, not(target_arch = "wasm32")))]
// mod tests {
//     use std::collections::{HashMap, HashSet};
//
//     use near_sdk::testing_env;
//
//     use nft_models::lemon::Lemon;
//     use nft_models::suppressor::Suppressor;
//     use nft_models::weapon::Weapon;
//     use test_utils::*;
//
//     use super::*;
//
//     const MINT_STORAGE_COST: u128 = 6_000_000_000_000_000_000_000;
//
//     #[test]
//     fn init() {
//         let mut context = get_context(bob());
//         testing_env!(context.build());
//         let contract = Contract::init(bob());
//         testing_env!(context.is_view(true).build());
//         assert_eq!(contract.nft_total_supply().0, 0);
//     }
//
//     #[test]
//     #[should_panic(expected = "The contract is not initialized")]
//     fn default() {
//         let context = get_context(bob());
//         testing_env!(context.build());
//         Contract::default();
//     }
//
//     #[test]
//     #[ignore]
//     fn mint_with_owner_contract() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .build());
//         let [token_id] = tokens::<1>();
//         let token = contract.mint(token_id.clone(), sample_token_metadata(), None);
//
//         let (metadata, model) = sample_token_metadata().split();
//
//         assert_eq!(token.token_id, token_id);
//         assert_eq!(token.owner_id, alice());
//         assert_eq!(token.metadata.unwrap(), metadata);
//         assert_eq!(token.model, model);
//         assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
//     }
//
//     #[test]
//     #[ignore]
//     fn mint_with_owner_id_is_not_contract_id() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .build());
//
//         let [token_id] = tokens::<1>();
//         let token = contract.mint(token_id.clone(), sample_token_metadata(), Some(bob()));
//         let (metadata, model) = sample_token_metadata().split();
//
//         assert_eq!(token.token_id, token_id);
//         assert_eq!(token.owner_id, bob());
//         assert_eq!(token.metadata.unwrap(), metadata);
//         assert_eq!(token.model, model);
//         assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
//     }
//
//     #[test]
//     #[ignore]
//     #[should_panic(expected = "The contract caller must be owner")]
//     fn mint_can_be_called_only_by_contract_owner() {
//         let mut context = get_context(bob());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .build());
//         let [token_id] = tokens::<1>();
//         contract.mint(token_id, sample_token_metadata(), Some(danny()));
//     }
//
//     #[test]
//     #[ignore]
//     fn get_owner_by_id_valid_and_it_is_contract_owner() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .build());
//         let [token_id] = tokens::<1>();
//         let token = contract.mint(token_id, sample_token_metadata(), None);
//         let owner_id = contract.get_owner_by_token_id(token.token_id).unwrap();
//         assert_eq!(owner_id, alice());
//     }
//
//     #[test]
//     #[ignore]
//     fn get_owner_by_id_valid_and_it_is_not_contract_owner() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .build());
//         let [token_id] = tokens::<1>();
//         let token = contract.mint(token_id, sample_token_metadata(), Some(danny()));
//         let owner_id = contract.get_owner_by_token_id(token.token_id).unwrap();
//         assert_eq!(owner_id, danny());
//     }
//
//     #[test]
//     #[ignore]
//     fn get_owner_by_id_invalid() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .build());
//         let [valid_token_id, invalid_token_id] = tokens::<2>();
//         contract.mint(valid_token_id, sample_token_metadata(), None);
//
//         assert!(contract.get_owner_by_token_id(invalid_token_id).is_none());
//     }
//
//     #[test]
//     #[ignore]
//     fn nft_tokens() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST * 4)
//             .build());
//
//         let [token_id1, token_id2, token_id3, token_id4] = tokens::<4>();
//
//         let foo_token0 = contract.mint(token_id1, foo_token_metadata_ext(), Some(bob()));
//         let foo_token1 = contract.mint(token_id2, foo_token_metadata_ext(), Some(danny()));
//         let baz_token0 = contract.mint(token_id3, baz_token_metadata_ext(), Some(carol()));
//         let baz_token1 = contract.mint(token_id4, baz_token_metadata_ext(), Some(fargo()));
//
//         let expected_tokens = vec![foo_token0, foo_token1, baz_token0, baz_token1];
//         let actual_tokens = contract.nft_tokens(None, None);
//         assert_eq!(expected_tokens.len(), actual_tokens.len());
//         assert!(expected_tokens.iter().all(|v| actual_tokens.contains(v)));
//
//         let vec_with_one_token = contract.nft_tokens(Some(U128(0)), Some(1));
//         assert_eq!(vec_with_one_token.len(), 1);
//         let vec_with_one_token = contract.nft_tokens(None, Some(1));
//         assert_eq!(vec_with_one_token.len(), 1);
//
//         let vec_with_three_tokens = contract.nft_tokens(Some(U128(1)), None);
//         assert_eq!(vec_with_three_tokens.len(), 3);
//
//         let vec_with_two_tokens = contract.nft_tokens(Some(U128(2)), Some(2));
//         assert_eq!(vec_with_two_tokens.len(), 2);
//     }
//
//     #[test]
//     #[ignore]
//     fn nft_total_supply() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST * 4)
//             .build());
//
//         let [token_id1, token_id2, token_id3, token_id4] = tokens::<4>();
//
//         contract.mint(token_id1, foo_token_metadata_ext(), Some(bob()));
//         assert_eq!(U128(1), contract.nft_total_supply());
//
//         contract.mint(token_id2, foo_token_metadata_ext(), Some(danny()));
//         assert_eq!(U128(2), contract.nft_total_supply());
//
//         contract.mint(token_id3, baz_token_metadata_ext(), Some(carol()));
//         assert_eq!(U128(3), contract.nft_total_supply());
//
//         contract.mint(token_id4, baz_token_metadata_ext(), Some(fargo()));
//         assert_eq!(U128(4), contract.nft_total_supply());
//     }
//
//     #[test]
//     #[ignore]
//     fn nft_token() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST * 2)
//             .build());
//
//         let [token_id1, token_id2] = tokens::<2>();
//         contract.mint(token_id1.clone(), foo_token_metadata_ext(), Some(bob()));
//         contract.mint(token_id2.clone(), baz_token_metadata_ext(), Some(danny()));
//
//         let actual_token_1 = contract.nft_token(token_id1.clone()).unwrap();
//         assert_eq!(actual_token_1.token_id, token_id1);
//         assert_eq!(actual_token_1.owner_id, bob());
//         assert_eq!(actual_token_1.model, foo_token_metadata_ext().model);
//
//         let actual_token_2 = contract.nft_token(token_id2.clone()).unwrap();
//         assert_eq!(actual_token_2.token_id, token_id2);
//         assert_eq!(actual_token_2.owner_id, danny());
//         assert_eq!(actual_token_2.model, baz_token_metadata_ext().model);
//     }
//
//     #[test]
//     #[ignore]
//     fn lemon_nft_transfer_change_owner_only_for_lemon_token_and_disassemble_it_from_weapon_nft() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST * 2)
//             .build());
//
//         let [lemon_id, weapon_id] = tokens::<2>();
//         let lemon = Lemon {
//             slots: [weapon_id.clone()].into(),
//             ..get_foo_lemon()
//         };
//         let weapon = Weapon {
//             slots: [lemon_id.clone()].into(),
//             ..get_foo_weapon()
//         };
//
//         contract.mint(lemon_id.clone(), fake_metadata_with(lemon), Some(bob()));
//         contract.mint(weapon_id.clone(), fake_metadata_with(weapon), Some(bob()));
//
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(1)
//             .predecessor_account_id(bob())
//             .build());
//
//         contract.nft_transfer(danny(), lemon_id.clone(), None, None);
//
//         match contract.nft_token(weapon_id.clone()).unwrap() {
//             TokenExt {
//                 owner_id,
//                 model: ModelKind::Weapon(Weapon { parent, .. }),
//                 ..
//             } => {
//                 assert_eq!(owner_id, bob());
//                 assert_eq!(parent, None);
//             }
//             _ => unreachable!(),
//         }
//
//         match contract.nft_token(lemon_id.clone()).unwrap() {
//             TokenExt {
//                 owner_id,
//                 model: ModelKind::Lemon(Lemon { slots, .. }),
//                 ..
//             } => {
//                 assert_eq!(owner_id, danny());
//                 assert_eq!(slots, HashSet::new());
//             }
//             _ => unreachable!(),
//         }
//     }
//
//     #[test]
//     fn weapon_nft_transfer_change_owner_only_for_weapon_token_and_disassemble_it_from_lemon_nft() {
//         let mut context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST * 2)
//             .build());
//
//         let [lemon_id, weapon_id, suppressor_id] = tokens::<3>();
//
//         let lemon = Lemon {
//             slots: [weapon_id.clone()].into(),
//             ..get_foo_lemon()
//         };
//
//         let suppressor = Suppressor {
//             parent: Some(weapon_id.clone()),
//             slots: HashSet::new(),
//         };
//
//         let weapon = Weapon {
//             parent: Some(lemon_id.clone()),
//             slots: [suppressor_id.clone()].into(),
//             ..get_foo_weapon()
//         };
//
//         contract.mint(lemon_id.clone(), fake_metadata_with(lemon), Some(bob()));
//         contract.mint(weapon_id.clone(), fake_metadata_with(weapon), Some(bob()));
//         contract.mint(
//             suppressor_id.clone(),
//             fake_metadata_with(suppressor),
//             Some(bob()),
//         );
//
//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(1)
//             .predecessor_account_id(bob())
//             .build());
//
//         contract.nft_transfer(danny(), weapon_id.clone(), None, None);
//
//         match contract.nft_token(weapon_id).unwrap() {
//             TokenExt {
//                 owner_id,
//                 model: ModelKind::Weapon(Weapon { parent, slots, .. }),
//                 ..
//             } => {
//                 assert_eq!(owner_id, danny());
//                 assert_eq!(parent, None);
//                 assert_eq!(slots, HashSet::new());
//             }
//             _ => unreachable!(),
//         }
//
//         match contract.nft_token(lemon_id).unwrap() {
//             TokenExt {
//                 owner_id,
//                 model: ModelKind::Lemon(Lemon { slots, .. }),
//                 ..
//             } => {
//                 assert_eq!(owner_id, bob());
//                 assert_eq!(slots, HashSet::new());
//             }
//             _ => unreachable!(),
//         }
//
//         match contract.nft_token(suppressor_id).unwrap() {
//             TokenExt {
//                 owner_id,
//                 model: ModelKind::Suppressor(Suppressor { parent, .. }),
//                 ..
//             } => {
//                 assert_eq!(owner_id, bob());
//                 assert_eq!(parent, None);
//             }
//             _ => unreachable!(),
//         }
//     }
//
//     #[test]
//     #[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
//     fn assemble_compound_nft_without_attached_deposit_panics() {
//         let context = get_context(alice());
//         testing_env!(context.build());
//         let mut contract = Contract::init(alice());
//         let instructions = vec![];
//         contract.assemble_compound_nft(instructions);
//     }
//
//     #[test]
//     #[should_panic(expected = "empty instructions are not allowed")]
//     fn assemble_compound_nft_empty_instructions() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(1).build());
//         let mut contract = Contract::init(alice());
//         let instructions = vec![];
//         contract.assemble_compound_nft(instructions);
//     }
//
//     #[test]
//     #[should_panic(expected = "Provided instructions contain errors")]
//     fn assemble_compound_nft_instructions_are_not_valid() {
//         let mut context = get_context(alice());
//         testing_env!(context.attached_deposit(1).build());
//         let mut contract = Contract::init(alice());
//         let instructions = vec!["1".to_string()];
//         contract.assemble_compound_nft(instructions);
//     }
// }
