use enum_dispatch::enum_dispatch;
use near_contract_standards::non_fungible_token::TokenId;

/// This trait collect all fields that contains `slot` in the name in `Vec<TokenId>`
#[enum_dispatch]
pub trait Slots {
    fn slots_id(self) -> Vec<TokenId>;
    fn take_slots(&mut self) -> Vec<Option<TokenId>>;
}
