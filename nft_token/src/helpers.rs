use crate::error::{BattlemonError as BtlError, Result};
use crate::Contract;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;
use nft_models::ModelKind;

impl Contract {
    pub(crate) fn owner(&self, body_id: &TokenId) -> Result<AccountId> {
        self.tokens
            .owner_by_id
            .get(&body_id)
            .ok_or_else(|| BtlError::OwnerNotFound(body_id.to_owned()))
    }

    pub(crate) fn model(&self, token_id: &TokenId) -> Result<ModelKind> {
        self.model_by_id
            .get(token_id)
            .ok_or_else(|| BtlError::ModelNotFound(token_id.to_owned()))
    }
}
