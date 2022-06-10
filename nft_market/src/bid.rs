use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(thiserror::Error, near_sdk::FunctionError, BorshSerialize, Debug)]
pub enum BidError {
    #[error("Bid is not found")]
    BidNotFound,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub token_id: TokenId,
    pub bidder_id: AccountId,
    pub price: U128,
}

// impl Bid {
//     pub fn new(token_id: TokenId, bidder_id: AccountId, price: u128) -> Self {
//         Self {
//             token_id,
//             bidder_id,
//             price: U128(price),
//         }
//     }
// }

impl crate::Contract {
    /// Add bid to the auction for concrete token.
    ///
    /// If asker already set as price less than bid, then
    /// bidder automatically wins the bid and get the asker's token
    /// the difference between bidder price and asker price must be returned to bidder
    pub(crate) fn add_bid(&self, bid: Bid) {
        let ask = self.ask_less_than_bid(&bid.token_id, bid.price);

        todo!();
    }

    pub(crate) fn bids(&self, token_id: TokenId) -> Vec<Bid> {
        todo!("get bids for token_id");
    }
}
