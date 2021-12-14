pub mod lemon;
mod slots;
pub mod weapon;
pub mod suppressor;
mod parent;

use enum_dispatch::enum_dispatch;
use lemon::Lemon;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
pub use slots::Slots;
pub use parent::Parent;
use weapon::Weapon;
use suppressor::Suppressor;

#[enum_dispatch(Slots, Parent)]
#[derive(Serialize, Deserialize, Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ModelKind {
    Lemon,
    Weapon,
    Suppressor,
}