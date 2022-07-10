use crate::Ask;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};

#[derive(thiserror::Error, near_sdk::FunctionError, BorshSerialize, Debug)]
pub enum BidError {
    #[error("Bid is not found")]
    BidNotFound,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub token_id: TokenId,
    pub expire_at: Option<u64>,
    pub account_id: AccountId,
    pub price: U128,
    pub create_at: u64,
}

impl Bid {
    pub(crate) fn new(token_id: TokenId, expire_at: Option<u64>) -> Self {
        Self {
            token_id,
            expire_at,
            account_id: env::predecessor_account_id(),
            price: U128(env::attached_deposit()),
            create_at: env::block_timestamp(),
        }
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn price(&self) -> u128 {
        self.price.0
    }

    pub fn token_id(&self) -> &TokenId {
        &self.token_id
    }

    pub fn create_at(&self) -> u64 {
        self.create_at
    }
}

impl crate::Contract {
    /// Add a bid to the auction to concrete the token.
    ///
    /// If the bid is more than the asker's token price,
    /// the bidder automatically gets the token.
    /// The difference between bidder and asker prices
    /// will be returned to the bidder by the market.

    pub(crate) fn highest_bid_than_ask(&self, ask: &Ask) -> Option<Bid> {
        let mut bids = self.bids.get(ask.token_id()).cloned().unwrap_or_default();
        bids.sort_unstable_by_key(|bid| (bid.price(), -(bid.create_at() as i128)));
        bids.pop()
    }

    pub(crate) fn count_bids_for_account(&self, account_id: &AccountId) -> usize {
        self.bids
            .values()
            .flatten()
            .filter(|bid| bid.account_id() == account_id)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::test_env::bob;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{serde_json, testing_env};

    #[test]
    fn deserialization_into_bid_works() {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(bob())
            .attached_deposit(777)
            .block_timestamp(666)
            .build());

        let bid: Bid = serde_json::from_str(r#"{"token_id":"1"}"#).unwrap();
        assert_eq!(bid.token_id(), "1");
        assert_eq!(*bid.account_id(), bob());
        assert_eq!(bid.price(), 777);
        assert_eq!(bid.create_at(), 666);
    }
}
