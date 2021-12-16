use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use crate::parent::Parent;
use parent_derive_macro::Parent;
use slots_derive_macro::Slots;

use crate::slots::Slots;

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Slots, Parent,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Suppressor {
    pub parent: Option<TokenId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suppressor_model() {
        let _suppressor = Suppressor { parent: "1".into() };
    }
}
