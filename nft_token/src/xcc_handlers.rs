use crate::error::ContractError;
use crate::{error::Result, Contract, ContractExt};
use battlemon_models::nft::{FromTraitWeights, Lemon, ModelKind};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{json_types::U128, AccountId, PromiseOrValue};

#[derive(near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct Message {
    tokens_ids: Vec<TokenId>,
}

#[near_sdk::ext_contract(ext_ft_receiver)]
pub trait FungibleTokenReceiver {
    /// Called by fungible token contract after `ft_transfer_call` was initiated by
    /// `sender_id` of the given `amount` with the transfer message given in `msg` field.
    /// The `amount` of tokens were already transferred to this contract account and ready to be used.
    ///
    /// The method must return the amount of tokens that are *not* used/accepted by this contract from the transferred
    /// amount. Examples:
    /// - The transferred amount was `500`, the contract completely takes it and must return `0`.
    /// - The transferred amount was `500`, but this transfer call only needs `450` for the action passed in the `msg`
    ///   field, then the method must return `50`.
    /// - The transferred amount was `500`, but the action in `msg` field has expired and the transfer must be
    ///   cancelled. The method must return `500` or panic.
    ///
    /// Arguments:
    /// - `sender_id` - the account ID that initiated the transfer.
    /// - `amount` - the amount of tokens that were transferred to this account in a decimal string representation.
    /// - `msg` - a string message that was passed with this transfer call.
    ///
    /// Returns the amount of unused tokens that should be returned to sender, in a decimal string representation.
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> Result<PromiseOrValue<U128>>;
}

#[near_sdk::near_bindgen]
impl FungibleTokenReceiver for Contract {
    #[handle_result]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> Result<PromiseOrValue<U128>> {
        // todo:
        // check that call from juice address

        // check that amount attached enough for crafting
        // check that tokens are craft-able
        // do craft logic with nft token (mint new with new properties, burn old)
        let message: Message = near_sdk::serde_json::from_str(&msg)?;

        for id in message.tokens_ids.iter() {
            let owner_id = self.owner(id)?;

            if owner_id != sender_id {
                return Err(ContractError::NotAuthorized(
                    "`sender_id` doesn't equal to token's owner.",
                ));
            }
        }

        let random = battlemon_models::helpers_contract::get_random_arr_range(0, 100);
        let token_id = self.new_token_id();
        let model = ModelKind::Lemon(Lemon::from_trait_weights(&token_id, &random));

        let token_metadata = near_contract_standards::non_fungible_token::metadata::TokenMetadata {
            title: Some("CraftedNft".to_string()),
            description: Some("It was created by craft".to_string()),
            media: Some(crate::consts::NFT_BACK_IMAGE.to_string()),
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

        self.internal_mint(token_id, sender_id, token_metadata, model);

        for id in message.tokens_ids.iter() {
            self.burn_token(id);
        }
        // todo: calculate change
        let change = U128(0);

        Ok(PromiseOrValue::Value(change))
    }
}
