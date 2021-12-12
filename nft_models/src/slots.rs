use enum_dispatch::enum_dispatch;
use near_contract_standards::non_fungible_token::TokenId;

#[enum_dispatch]
pub trait Slots {
    fn slots_id(&self) -> Vec<&TokenId>;
}
