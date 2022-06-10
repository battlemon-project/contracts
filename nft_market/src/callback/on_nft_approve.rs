use crate::{Ask, Contract, ContractError, ContractExt};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PromiseOrValue};

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
        msg: Action,
    ) -> Result<PromiseOrValue<String>, ContractError>;
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum Action {
    /// Create ask order for token with provided price.
    AddAsk { price: U128 },
    /// Token owner accept the bid with the biggest price.
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
        msg: Action,
    ) -> Result<PromiseOrValue<String>, ContractError> {
        if env::predecessor_account_id() != self.nft_id {
            return Err(ContractError::NotAuthorized);
        }

        let ret = match msg {
            Action::AddAsk { price } => {
                self.add_ask(Ask::new(owner_id, token_id, approval_id, price))?
            }
            Action::AcceptBid => {
                todo!("call promise with accepting bid")
            }
        };

        Ok(ret)
        // let SaleArgs { sale_type, price } =
        //     serde_json::from_str(&msg).expect("Couldn't parse json");
        //
        // match sale_type {
        //     SaleType::AcceptBid => {
        //         let promise = self.process_purchase(
        //             token_id,
        //             OrderType::AcceptBid {
        //                 owner_id,
        //                 approval_id,
        //             },
        //         );
        //
        //         PromiseOrValue::Promise(promise)
        //     }
        //     SaleType::Selling => {
        //         let price = price.expect("The price didn't provided for selling").0;
        //         let sale_conditions =
        //             SaleCondition::new(owner_id, token_id.clone(), approval_id, price);
        //         self.asks.insert(&token_id, &sale_conditions);
        //         let ret = json!({
        //             "status": true,
        //             "message": format!("token {} with price {} was added to market", token_id, price)
        //         });
        //
        //         Ok(PromiseOrValue::Value(ret.to_string()))
        //     }
        // }
    }
}
