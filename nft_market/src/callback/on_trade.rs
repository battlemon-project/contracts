use crate::{Ask, Bid, Contract, ContractExt};
use near_sdk::{near_bindgen, Promise};

#[near_bindgen]
impl Contract {
    #[private]
    pub fn on_trade(&mut self, ask: Ask, bid: Bid, change: bool, #[callback_unwrap] _trade: ()) {
        let to_asker = Promise::new(ask.account_id().to_owned()).transfer(ask.price());
        if change {
            let diff = bid.price() - ask.price();
            let to_bidder = Promise::new(bid.account_id().to_owned()).transfer(diff);
            to_asker.and(to_bidder);
        }
    }
}
