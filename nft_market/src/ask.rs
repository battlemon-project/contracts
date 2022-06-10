use crate::ContractError;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, PromiseOrValue};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Ask {
    pub owner_id: AccountId,
    pub token_id: TokenId,
    approval_id: u64,
    pub price: U128,
}

impl Ask {
    pub fn new(owner_id: AccountId, token_id: TokenId, approval_id: u64, price: U128) -> Self {
        Self {
            owner_id,
            token_id,
            approval_id,
            price,
        }
    }
}

impl crate::Contract {
    /// Add ask for a concrete token.
    ///
    /// The market automatically completes the trade
    /// if the asker provides a price less than the highest bid.
    /// First, the bidder receives the asker's token.
    /// Then, the asker gets the bidder's nears held by the market.
    pub(crate) fn add_ask(&self, ask: Ask) -> Result<PromiseOrValue<String>, ContractError> {
        todo!("add ask for token_id");
    }

    pub(crate) fn ask(&self, token_id: &TokenId) -> Option<Ask> {
        self.asks.get(token_id)
    }

    pub(crate) fn ask_less_than_bid(&self, token_id: &TokenId, price: U128) -> Option<Ask> {
        self.ask(token_id).filter(|ask| ask.price < price)
    }
}

#[cfg(all(test, not(target = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::accounts;

    #[test]
    fn add_ask_works() {
        let contract = crate::Contract::init(accounts(1));
        let ask = Ask {
            owner_id: accounts(2),
            token_id: "1".to_string(),
            approval_id: 0,
            price: U128(100),
        };

        contract.add_ask(ask);
    }
}
