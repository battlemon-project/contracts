use crate::Contract;
use battlemon_models::nft::ModelKind;
use near_contract_standards::non_fungible_token::{
    core::StorageKey, events::NftMint, metadata::TokenMetadata, refund_deposit_to_account, TokenId,
};
use near_sdk::{collections::UnorderedSet, env, AccountId};
use std::collections::HashMap;
use token_metadata_ext::TokenExt;

impl Contract {
    pub(crate) fn new_token_id(&mut self) -> TokenId {
        self.last_token_id += 1;
        self.last_token_id.to_string()
    }

    /// Mint a new token without checking:
    /// * Whether the caller id is equal to the `owner_id`
    /// * Assumes there will be a refund to the predecessor after covering the storage costs
    ///
    /// Returns the newly minted token and emits the mint event
    pub(crate) fn internal_mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        model: ModelKind,
    ) -> TokenExt {
        let token = self.internal_mint_with_refund(
            token_id,
            token_owner_id,
            token_metadata,
            env::predecessor_account_id(),
            model,
        );

        NftMint {
            owner_id: &token.owner_id,
            token_ids: &[&token.token_id],
            memo: None,
        }
        .emit();

        token
    }

    /// Mint a new token without checking:
    /// * Whether the caller id is equal to the `owner_id`
    /// * Assumes there will be a refund to the predecessor after covering the storage costs
    ///
    /// Returns the newly minted token and emits the mint event
    pub(crate) fn internal_mint_full(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        model: ModelKind,
    ) -> TokenExt {
        let token = self.internal_mint_without_refund(
            token_id,
            token_owner_id,
            token_metadata,
            model,
        );

        NftMint {
            owner_id: &token.owner_id,
            token_ids: &[&token.token_id],
            memo: None,
        }
        .emit();

        token
    }
    /// Mint a new token without checking:
    /// * Whether the caller id is equal to the `owner_id`
    /// * `refund_id` will transfer the left over balance after storage costs are calculated to the provided account.
    ///   Typically the account will be the owner. If `None`, will not refund. This is useful for delaying refunding
    ///   until multiple tokens have been minted.
    ///
    /// Returns the newly minted token and does not emit the mint event. This allows minting multiple before emitting.
    pub(crate) fn internal_mint_without_refund(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        model: ModelKind,
    ) -> TokenExt {
        self.model_by_id.insert(&token_id, &model);
        self.tokens.owner_by_id.insert(&token_id, &token_owner_id);

        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &token_metadata));

        // Enumeration extension: Record tokens_per_owner for use with enumeration view methods.
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&token_owner_id).unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TokensPerOwner {
                    account_hash: env::sha256(token_owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&token_owner_id, &token_ids);
        }

        // Approval Management extension: return empty HashMap as part of Token
        let approved_account_ids = self.tokens.approvals_by_id.is_some().then(HashMap::new);

        TokenExt {
            token_id,
            owner_id: token_owner_id,
            metadata: Some(token_metadata),
            model,
            approved_account_ids,
        }
    }
    /// Mint a new token without checking:
    /// * Whether the caller id is equal to the `owner_id`
    /// * `refund_id` will transfer the left over balance after storage costs are calculated to the provided account.
    ///   Typically the account will be the owner. If `None`, will not refund. This is useful for delaying refunding
    ///   until multiple tokens have been minted.
    ///
    /// Returns the newly minted token and does not emit the mint event. This allows minting multiple before emitting.
    pub(crate) fn internal_mint_with_refund(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        refund_id: AccountId,
        model: ModelKind,
    ) -> TokenExt {
        let initial_storage_usage = env::storage_usage();

        self.model_by_id.insert(&token_id, &model);
        self.tokens.owner_by_id.insert(&token_id, &token_owner_id);

        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &token_metadata));

        // Enumeration extension: Record tokens_per_owner for use with enumeration view methods.
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&token_owner_id).unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TokensPerOwner {
                    account_hash: env::sha256(token_owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&token_owner_id, &token_ids);
        }

        // Approval Management extension: return empty HashMap as part of Token
        let approved_account_ids = self.tokens.approvals_by_id.is_some().then(HashMap::new);

        refund_deposit_to_account(env::storage_usage() - initial_storage_usage, refund_id);

        TokenExt {
            token_id,
            owner_id: token_owner_id,
            metadata: Some(token_metadata),
            model,
            approved_account_ids,
        }
    }
}
