use crate::Bid;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Ask {
    pub account_id: AccountId,
    pub token_id: TokenId,
    pub approval_id: u64,
    pub price: U128,
}

impl Ask {
    pub fn new(owner_id: AccountId, token_id: TokenId, approval_id: u64, price: U128) -> Self {
        Self {
            account_id: owner_id,
            token_id,
            approval_id,
            price,
        }
    }

    pub(crate) fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub(crate) fn approval_id(&self) -> Option<u64> {
        Some(self.approval_id)
    }

    pub fn token_id(&self) -> &TokenId {
        &self.token_id
    }

    pub(crate) fn price(&self) -> u128 {
        self.price.0
    }
}

impl crate::Contract {
    /// Add ask for a concrete token.
    ///
    /// The market automatically completes the trade
    /// if the asker provides a price less than the highest bid.
    /// First, the bidder receives the asker's token.
    /// Then, the asker gets the bidder's Nears held by the market.
    pub(crate) fn add_ask(&mut self, ask: &Ask) {
        match self.highest_bid_than_ask(ask) {
            None => {
                self.asks.insert(ask.token_id().to_owned(), ask.to_owned());
            }
            Some(bid) => self.trade(ask.to_owned(), bid, false),
        }
    }

    pub(crate) fn ask_less_than_bid(&self, bid: &Bid) -> Option<Ask> {
        self.asks
            .get(bid.token_id())
            .filter(|ask| ask.price() <= bid.price())
            .cloned()
    }

    pub(crate) fn count_asks_for_account(&self, account_id: &AccountId) -> usize {
        self.asks
            .iter()
            .filter(|(_, ask)| ask.account_id() == account_id)
            .count()
    }
}
