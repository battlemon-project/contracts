use crate::{helpers, serde_json, Ask, Contract, ContractError, ContractExt};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ext_contract, near_bindgen, AccountId};

#[ext_contract(ext_nft_approval_receiver)]
pub trait NonFungibleTokenApprovalReceiver {
    /// Respond to notification that contract has been granted approval for a token.
    ///
    /// Notes
    /// * Contract knows the token contract ID from `predecessor_account_id`
    ///
    /// Arguments:
    /// * `token_id`: the token to which this contract has been granted approval
    /// * `owner_id`: the owner of the token
    /// * `approval_id`: the approval ID stored by NFT contract for this approval.
    ///   Expected to be a number within the 2^53 limit representable by JSON.
    /// * `msg`: specifies information needed by the approved contract in order to
    ///    handle the approval. Can indicate both a function to call and the
    ///    parameters to pass to that function.
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> Result<(), ContractError>;
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum Action {
    /// Create ask order for token with provided price.
    AddAsk { price: U128 },
    /// The token owner accepts the bid with the biggest price.
    AcceptBid,
}

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Contract {
    #[handle_result]
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> Result<(), ContractError> {
        helpers::check_cross_contract_call(&self.nft_id)?;
        self.check_storage_deposits(&owner_id)?;

        match serde_json::from_str(&*msg)? {
            Action::AddAsk { price } => {
                self.add_ask(&Ask::new(owner_id, token_id, approval_id, price))
            }
            Action::AcceptBid => {
                todo!("call promise with accepting bid")
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::serde_json;

    #[test]
    fn enum_action_deserialization_works_for_add_ask() {
        let msg = r#"{"action":"add_ask","price":"1000"}"#;
        serde_json::from_str::<Action>(msg).expect("Failed to deserialization");
    }
}
