use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;

use battlemon_models::nft::ModelKind;

use crate::error::{ContractError, Result};
use crate::Contract;

impl Contract {
    pub(crate) fn owner(&self, id: &TokenId) -> Result<AccountId> {
        self.tokens
            .owner_by_id
            .get(id)
            .ok_or_else(|| ContractError::OwnerNotFound(id.to_owned()))
    }

    pub(crate) fn model(&self, id: &TokenId) -> Result<ModelKind> {
        self.model_by_id
            .get(id)
            .ok_or_else(|| ContractError::ModelNotFound(id.to_owned()))
    }

    pub(crate) fn burn_token(&mut self, token_id: &TokenId) {
        self.model_by_id.remove(token_id);
        let tokens = &mut self.tokens;
        let owner_id = tokens.owner_by_id.remove(token_id).unwrap();

        if let Some(collection) = &mut tokens.token_metadata_by_id {
            collection.remove(token_id);
        }

        if let Some(collection) = &mut tokens.tokens_per_owner {
            let mut tokens = collection.get(&owner_id).unwrap();
            tokens.remove(token_id);
            if tokens.is_empty() {
                collection.remove(&owner_id);
            } else {
                collection.insert(&owner_id, &tokens);
            }
        }

        if let Some(collection) = &mut tokens.approvals_by_id {
            collection.remove(token_id);
        }

        if let Some(collection) = &mut tokens.next_approval_id_by_id {
            collection.remove(token_id);
        }

        near_contract_standards::non_fungible_token::events::NftBurn {
            owner_id: &owner_id,
            token_ids: &[token_id],
            authorized_id: None,
            memo: None,
        }
        .emit();
    }
}
