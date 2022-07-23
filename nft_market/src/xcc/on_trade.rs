use crate::{consts, Ask, Bid, Contract, ContractExt};
use consts::EVENT_PREFIX;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Promise, PromiseError};

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
        let trade_price;
        if change {
            trade_price = ask.price();
            let diff = bid.price() - trade_price;
            let to_bidder = Promise::new(bid.account_id().to_owned()).transfer(diff);
            to_asker.transfer(trade_price).and(to_bidder);
        } else {
            trade_price = bid.price();
            to_asker.transfer(trade_price);
        }

        self.clean_ask_and_bid(&bid);
        let json = near_sdk::serde_json::json!({
           "prev_owner": ask.account_id(),
           "curr_owner": bid.account_id(),
           "token_id": ask.token_id(),
           "price": U128(trade_price),
        });
        env::log_str(&format!("{EVENT_PREFIX}:{json}"));
    }
}
