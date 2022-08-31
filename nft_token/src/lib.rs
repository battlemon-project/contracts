use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::env::{self, panic_str};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, require, AccountId, PanicOnDefault, Promise};

use crate::consts::{DATA_IMAGE_SVG_LEMON_LOGO, IPFS_GATEWAY_BASE_URL, NFT_BACK_IMAGE};
use battlemon_models::helpers_contract::weights;
use battlemon_models::nft::{
    Back, Cap, Cloth, ColdArm, FireArm, FromTraitWeights, Lemon, ModelKind, NftKind,
};
use token_metadata_ext::TokenExt;

mod consts;
mod error;
mod helpers;
mod internal;
mod mint;
mod xcc_handlers;

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
            base_uri: Some(IPFS_GATEWAY_BASE_URL.to_string()),
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();

        Self::new(owner_id, metadata)
    }

    #[payable]
    pub fn nft_mint(&mut self, receiver_id: AccountId, kind: NftKind) -> TokenExt {
        let model = match kind {
            NftKind::Lemon => ModelKind::Lemon(Lemon::from_trait_weights(&weights())),
            NftKind::Firearm => ModelKind::FireArm(FireArm::from_trait_weights(&weights())),
            NftKind::Coldarm => ModelKind::ColdArm(ColdArm::from_trait_weights(&weights())),
            NftKind::Cloth => ModelKind::Cloth(Cloth::from_trait_weights(&weights())),
            NftKind::Back => ModelKind::Back(Back::from_trait_weights(&weights())),
            NftKind::Cap => ModelKind::Cap(Cap::from_trait_weights(&weights())),
        };

        let token_metadata = TokenMetadata {
            title: None,
            description: None,
            media: Some(NFT_BACK_IMAGE.to_string()),
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        self.internal_mint(receiver_id, token_metadata, model)
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
        require!(
            env::predecessor_account_id() == self.tokens.owner_id,
            "Unauthorized"
        );

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
    //

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
