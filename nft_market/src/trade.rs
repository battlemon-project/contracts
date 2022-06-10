use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, AccountId};

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum TradeType {
    Sell,
    Buy,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub prev_owner: AccountId,
    pub curr_owner: AccountId,
    pub price: u128,
    pub date: near_sdk::Timestamp,
    #[serde(rename = "type")]
    pub type_: TradeType,
}

impl Trade {
    pub fn from_sale(sale: crate::ask::Ask, curr_owner: AccountId, type_: TradeType) -> Self {
        Self {
            prev_owner: sale.owner_id,
            curr_owner,
            price: sale.price.0,
            date: env::block_timestamp(),
            type_,
        }
    }
}
