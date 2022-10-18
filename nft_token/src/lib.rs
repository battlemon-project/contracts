use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use near_contract_standards::non_fungible_token::events::NftMint;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{
    refund_deposit_to_account, NonFungibleToken, TokenId,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::env::{self, panic_str};
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, near_bindgen, require, AccountId, PanicOnDefault, Promise};

use crate::consts::{DATA_IMAGE_SVG_LEMON_LOGO, IPFS_GATEWAY_BASE_URL, NFT_BACK_IMAGE};
use battlemon_models::helpers_contract::{emit_log_event, weights};
use battlemon_models::nft::{
    Back, Cap, Cloth, ColdArm, FireArm, FromTraitWeights, Lemon, ModelKind, NftEvent, NftEventKind,
    NftKind, Set, StandardKind, TokenExt, VersionKind,
};

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
        let token_id = self.new_token_id();

        let model = match kind {
            NftKind::Lemon => ModelKind::Lemon(Lemon::from_trait_weights(&token_id, &weights())),
            NftKind::FireArm => {
                ModelKind::FireArm(FireArm::from_trait_weights(&token_id, &weights()))
            }
            NftKind::ColdArm => {
                ModelKind::ColdArm(ColdArm::from_trait_weights(&token_id, &weights()))
            }
            NftKind::Cloth => ModelKind::Cloth(Cloth::from_trait_weights(&token_id, &weights())),
            NftKind::Back => ModelKind::Back(Back::from_trait_weights(&token_id, &weights())),
            NftKind::Cap => ModelKind::Cap(Cap::from_trait_weights(&token_id, &weights())),
            NftKind::Set => ModelKind::Set(Set::from_trait_weights(&token_id, &weights())),
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

        self.internal_mint(token_id, receiver_id, token_metadata, model)
    }

    #[payable]
    pub fn nft_mint_full(&mut self, receiver_id: AccountId) -> Vec<TokenExt> {
        require!(
            env::prepaid_gas() >= near_sdk::Gas(40_000_000_000_000),
            format!(
                "Not enough gas for full mint, attached gas is {:?}",
                env::prepaid_gas()
            )
        );
        let fire_arm_token_id = self.new_token_id();
        let fire_arm_model =
            ModelKind::FireArm(FireArm::from_trait_weights(&fire_arm_token_id, &weights()));

        let cold_arm_token_id = self.new_token_id();
        let cold_arm_model =
            ModelKind::ColdArm(ColdArm::from_trait_weights(&cold_arm_token_id, &weights()));

        let cloth_token_id = self.new_token_id();
        let cloth_model = ModelKind::Cloth(Cloth::from_trait_weights(&cloth_token_id, &weights()));

        let back_token_id = self.new_token_id();
        let back_model = ModelKind::Back(Back::from_trait_weights(&back_token_id, &weights()));

        let cap_token_id = self.new_token_id();
        let cap_model = ModelKind::Cap(Cap::from_trait_weights(&cap_token_id, &weights()));

        let lemon_token_id = self.new_token_id();
        let lemon_model = ModelKind::Lemon(Lemon::from_trait_weights(&lemon_token_id, &weights()));

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

        let parts = vec![
            (fire_arm_token_id, fire_arm_model),
            (cold_arm_token_id, cold_arm_model),
            (cloth_token_id, cloth_model),
            (back_token_id, back_model),
            (cap_token_id, cap_model),
            (lemon_token_id, lemon_model),
        ];

        let initial_storage_usage = env::storage_usage();

        for (token_id, model) in parts.iter() {
            self.internal_mint_full(
                token_id.clone(),
                receiver_id.clone(),
                token_metadata.clone(),
                model.clone(),
            );
        }
        let tokens_ids: Vec<_> = parts.iter().map(|(id, _)| id.clone()).collect();
        let tokens_ids_str: Vec<_> = tokens_ids.iter().map(String::as_str).collect();

        NftMint {
            owner_id: &receiver_id,
            token_ids: &tokens_ids_str,
            memo: None,
        }
        .emit();

        let (lemon_id, other_ids) = tokens_ids.split_last().unwrap();
        for id in other_ids {
            self.merge_ids(lemon_id, id);
        }

        refund_deposit_to_account(
            env::storage_usage() - initial_storage_usage,
            env::predecessor_account_id(),
        );

        tokens_ids
            .into_iter()
            .flat_map(|id| self.nft_token(id))
            .collect()
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
            .nft_transfer(receiver_id, token_id.clone(), approval_id, memo);
        self.disassemble_all(&token_id);
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

    #[payable]
    pub fn assemble_compound_nft(&mut self, instructions: Vec<TokenId>) -> TokenExt {
        assert_one_yocto();
        self.check_instructions(&instructions)
            .expect("Provided instructions contain errors");

        let (lemon_id, other_ids) = instructions.split_first().unwrap();
        for id in other_ids {
            self.merge_ids(lemon_id, id);
        }

        let event = NftEvent {
            standard: StandardKind::Nep171,
            version: VersionKind::V1_0_0,
            event: NftEventKind::AssembleNft,
            data: None,
        };

        emit_log_event(event);
        self.nft_token(lemon_id.clone()).unwrap()
    }

    #[payable]
    pub fn disassemble_compound_nft(&mut self, instructions: Vec<TokenId>) -> TokenExt {
        assert_one_yocto();
        self.check_instructions(&instructions)
            .expect("Provided instructions contain errors");

        let (lemon_id, other_ids) = instructions.split_first().unwrap();
        for id in other_ids {
            self.unmerge_ids(lemon_id, id);
        }

        let event = NftEvent {
            standard: StandardKind::Nep171,
            version: VersionKind::V1_0_0,
            event: NftEventKind::DisassembleNft,
            data: None,
        };

        emit_log_event(event);
        self.nft_token(lemon_id.clone()).unwrap()
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().expect("Metadata didn't set")
    }
}

near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
