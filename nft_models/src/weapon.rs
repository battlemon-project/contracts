use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use crate::parent::Parent;
use crate::slots::Slots;
use parent_derive_macro::Parent;
use slots_derive_macro::Slots;

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Slots, Parent,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Weapon {
    pub level: u8,
    pub r#type: Type,
    pub parent: Option<TokenId>,
    pub scope_slot: Option<TokenId>,
    pub perk_slot: Option<TokenId>,
    pub mag_slot: Option<TokenId>,
    pub barrel_slot: Option<TokenId>,
    pub muzzle_slot: Option<TokenId>,
    pub grip_slot: Option<TokenId>,
    pub stock_slot: Option<TokenId>,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Type {
    Instant,
    Projection,
    Collusion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_model() {
        let _weapon = Weapon {
            level: 0,
            r#type: Type::Instant,
            parent: None,
            scope_slot: None,
            perk_slot: None,
            mag_slot: None,
            barrel_slot: None,
            muzzle_slot: None,
            grip_slot: None,
            stock_slot: None,
        };
    }
}
