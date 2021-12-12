pub mod lemon;
mod slots;
pub mod weapon;

use enum_dispatch::enum_dispatch;
use lemon::Lemon;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
pub use slots::Slots;
use weapon::Weapon;

#[enum_dispatch(Slots)]
#[derive(Serialize, Deserialize, Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ModelKind {
    Lemon,
    Weapon,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
