use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use slots_derive_macro::Slots;

use crate::slots::Slots;

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Slots,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Suppressor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suppressor_model() {
        let _suppressor = Suppressor {};
    }
}
