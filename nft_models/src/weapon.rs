use crate::manager::Manager;
use manager_derive_macro::Manager;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Manager,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Weapon {
    pub level: u8,
    pub r#type: Type,
    pub parent: Option<TokenId>,
    pub slots: HashSet<TokenId>,
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
            slots: HashSet::new(),
        };
    }
}
