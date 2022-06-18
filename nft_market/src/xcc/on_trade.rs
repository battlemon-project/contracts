use crate::{Ask, Bid, Contract, ContractExt};
use near_sdk::{near_bindgen, Promise, PromiseError};

#[near_bindgen]
impl Contract {
    #[private]
    pub fn on_trade(
        &mut self,
        ask: Ask,
        bid: Bid,
        change: bool,
        #[callback_result] trade: Result<(), PromiseError>,
    ) {
        trade.unwrap();
        let to_asker = Promise::new(ask.account_id().to_owned());
        if change {
            let diff = bid.price() - ask.price();
            let to_bidder = Promise::new(bid.account_id().to_owned()).transfer(diff);
            to_asker.transfer(ask.price()).and(to_bidder);
        } else {
            to_asker.transfer(bid.price());
        }

        self.clean_ask_and_bid(&bid);
    }
}
