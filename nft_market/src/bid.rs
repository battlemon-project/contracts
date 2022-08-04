use battlemon_models::market::ask_contract::Ask;
use battlemon_models::market::bid_contract::Bid;
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::AccountId;

#[derive(thiserror::Error, near_sdk::FunctionError, BorshSerialize, Debug)]
pub enum BidError {
    #[error("Bid is not found")]
    BidNotFound,
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
