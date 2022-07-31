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
        let trade_for_log = battlemon_models::market::SaleForContract {
            prev_owner: ask.account_id().to_string(),
            curr_owner: bid.account_id().to_string(),
            token_id: ask.token_id().to_string(),
            price: U128(trade_price),
        };

        let json = near_sdk::serde_json::to_value(&trade_for_log).unwrap();
        env::log_str(&format!("{EVENT_PREFIX}:{json}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sale_for_contract_valid_logs() {
        let expected_json = near_sdk::serde_json::json!({
            "prev_owner": "alice.near",
            "curr_owner": "bob.near",
            "token_id": "1",
            "price": U128(10000000000000000000000),
        });

        let sale_for_contract = battlemon_models::market::SaleForContract {
            prev_owner: "alice.near".to_string(),
            curr_owner: "bob.near".to_string(),
            token_id: "1".to_string(),
            price: U128(10000000000000000000000),
        };

        let actual_json = near_sdk::serde_json::to_value(&sale_for_contract).unwrap();

        assert_eq!(actual_json, expected_json);
    }
}
