pub mod lemon;
pub mod manager;
pub mod suppressor;
pub mod weapon;

use enum_dispatch::enum_dispatch;
use lemon::Lemon;
pub use manager::Manager;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use suppressor::Suppressor;
use weapon::Weapon;

#[enum_dispatch(Manager)]
#[derive(Serialize, Deserialize, Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ModelKind {
    Lemon,
    Weapon,
    Suppressor,
}

impl ModelKind {
    fn is_compatible(&self, model_kind: &Self) -> bool {
        match (self, model_kind) {
            (ModelKind::Lemon(_), ModelKind::Weapon(_)) => true,
            (ModelKind::Weapon(_), ModelKind::Suppressor(_)) => true,
            _ => false,
        }
    }
}
