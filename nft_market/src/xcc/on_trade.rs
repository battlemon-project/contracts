use crate::{helpers, Contract, ContractExt};
use battlemon_models::market::events::MarketEventKind;
use battlemon_models::market::{ask::AskForContract, bid::BidForContract};
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, Promise, PromiseError};

#[near_bindgen]
impl Contract {
    #[private]
    pub fn on_trade(
        &mut self,
        ask: AskForContract,
        bid: BidForContract,
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

        let trade_for_log = battlemon_models::market::sale::SaleForContract {
            prev_owner: ask.account_id().to_string(),
            curr_owner: bid.account_id().to_string(),
            token_id: ask.token_id().to_string(),
            price: U128(trade_price),
        };

        helpers::emit_log_event(MarketEventKind::Sale(trade_for_log));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sale_for_contract_valid_logs() {
        let expected_json = near_sdk::serde_json::json!({
            "event": "sale",
            "data": {
                "prev_owner": "alice.near",
                "curr_owner": "bob.near",
                "token_id": "1",
                "price": U128(10000000000000000000000),
            }
        });

        let sale_for_contract = battlemon_models::market::sale::SaleForContract {
            prev_owner: "alice.near".to_string(),
            curr_owner: "bob.near".to_string(),
            token_id: "1".to_string(),
            price: U128(10000000000000000000000),
        };
        let event = MarketEventKind::Sale(sale_for_contract);
        let actual_json = near_sdk::serde_json::to_value(&event).unwrap();

        assert_eq!(actual_json, expected_json);
    }
}
