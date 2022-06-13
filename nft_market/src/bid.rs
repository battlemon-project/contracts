use crate::{Ask, ContractError};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::block_timestamp;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Deserializer, Serialize};
use near_sdk::{env, AccountId};

#[derive(thiserror::Error, near_sdk::FunctionError, BorshSerialize, Debug)]
pub enum BidError {
    #[error("Bid is not found")]
    BidNotFound,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    token_id: TokenId,
    #[serde(default = "env::predecessor_account_id")]
    account_id: AccountId,
    #[serde(default = "attached_deposit")]
    price: U128,
    expire_at: Option<u64>,
    #[serde(default = "env::block_timestamp")]
    create_at: u64,
}

fn attached_deposit() -> U128 {
    env::attached_deposit().into()
}

impl Bid {
    pub(crate) fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub(crate) fn price(&self) -> u128 {
        self.price.0
    }

    pub(crate) fn token_id(&self) -> &TokenId {
        &self.token_id
    }

    pub(crate) fn create_at(&self) -> u64 {
        self.create_at
    }
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
    /// Add a bid to the auction to concrete the token.
    ///
    /// If the bid is more than the asker's token price,
    /// the bidder automatically gets the token.
    /// The difference between bidder and asker prices
    /// will be returned to the bidder by the market.
    pub(crate) fn add_bid(&mut self, bid: Bid) {
        match self.ask_less_than_bid(&bid) {
            None => {
                self.bids
                    .entry(bid.token_id().clone())
                    .and_modify(|bids| {
                        bids.push(bid.clone());
                    })
                    .or_insert_with(|| vec![bid]);
            }
            Some(ask) => self.trade(ask, bid, true),
        }
    }

    pub(crate) fn find_highest_bid_than_ask(&self, ask: &Ask) -> Option<Bid> {
        let mut bids = self.bids.get(ask.token_id()).cloned().unwrap_or_default();
        bids.sort_by_key(|bid| (bid.price(), -(bid.create_at() as i128)));
        bids.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::serde_json::json;
    use near_sdk::test_utils::test_env::{alice, bob};
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
        assert_eq!(bid.price(), U128(777));
        assert_eq!(bid.create_at(), 666);
    }

    // #[test]
    // fn bids_works() {
    //     let mut contract = crate::Contract::init(alice());
    //     let token_id = "1".to_owned();
    //     let bid = Bid {
    //         token_id: token_id.clone(),
    //         bidder_id: bob(),
    //         price: U128(1),
    //     };
    //     contract.bids.insert(&token_id, &vec![bid.clone()]);
    //     assert_eq!(contract.bids_owned(bid.token_id()), vec![bid]);
    // }
    //
    // #[test]
    // fn add_bid_works() {
    //     let mut contract = crate::Contract::init(alice());
    //
    //     let bid = Bid {
    //         token_id: "1".to_string(),
    //         bidder_id: bob(),
    //         price: U128(1),
    //     };
    //
    //     contract.add_bid(&bid);
    // }
}
