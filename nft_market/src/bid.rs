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
    /// The market will return the difference between bidder and asker prices to the bidder.

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
