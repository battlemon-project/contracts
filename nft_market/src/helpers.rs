use crate::{ContractError, STORAGE_PER_SALE};
use battlemon_models::market::bid::BidForContract;
use battlemon_models::market::events::MarketEventKind;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};

impl crate::Contract {
    pub(crate) fn total_orders_by_id(&self, id: &AccountId) -> usize {
        self.count_bids_for_account(id) + self.count_asks_for_account(id)
    }

    pub(crate) fn check_storage_deposits(&self, id: &AccountId) -> Result<(), ContractError> {
        let paid_storage = self.storage_deposits.get(id).copied().unwrap_or_default();

        let required_storage = (self.total_orders_by_id(id) + 1) as u128 * STORAGE_PER_SALE;

        if paid_storage < required_storage {
            return Err(ContractError::StorageError(
                "Not enough storage deposits to create new order",
            ));
        }

        Ok(())
    }

    pub(crate) fn clean_ask_and_bid(&mut self, bid: &BidForContract) {
        let token_id = bid.token_id();
        if let Some(ask) = self.asks.remove(token_id) {
            emit_log_event(MarketEventKind::RemoveAsk(ask))
        }

        if let Some(bids) = self.bids.get_mut(token_id) {
            emit_log_and_and_remove_bid(&bid.id, bids);
            bids.is_empty().then(|| self.bids.remove(token_id));
        }
    }
}

fn emit_log_and_and_remove_bid(id: &str, bids: &mut Vec<BidForContract>) {
    if let Some(idx) = bids.iter().position(|b| b.id == id) {
        let bid = bids.swap_remove(idx);
        emit_log_event(MarketEventKind::RemoveBid(bid));
    }
}

pub fn check_cross_contract_call(id: &AccountId) -> Result<(), ContractError> {
    if env::predecessor_account_id() != *id {
        return Err(ContractError::NotAuthorized(
            "The predecessor account id isn't allowed",
        ));
    }
    Ok(())
}

pub fn emit_log_event<'de>(model: impl Deserialize<'de> + Serialize) {
    let value = near_sdk::serde_json::to_value(model).unwrap();
    env::log_str(&format!("{}:{value}", crate::EVENT_PREFIX));
}
