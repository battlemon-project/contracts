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
pub struct Suppressor {
    pub parent: Option<TokenId>,
    pub slots: HashSet<TokenId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suppressor_model() {
        let _suppressor = Suppressor {
            parent: None,
            slots: HashSet::new(),
        };
    }
}
