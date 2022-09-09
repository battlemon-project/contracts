use crate::error::Result;
use crate::Contract;
use battlemon_models::nft::TokenExt;
use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::{AccountId, BorshStorageKey};

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

    pub(crate) fn collect_ext_tokens(&self, tokens: Vec<Token>) -> Result<Vec<TokenExt>> {
        tokens
            .into_iter()
            .map(|token| {
                let model = self.model(&token.token_id)?;
                Ok(TokenExt::from_parts(token, model))
            })
            .collect()
    }
}
