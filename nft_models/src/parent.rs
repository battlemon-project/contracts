use near_contract_standards::non_fungible_token::TokenId;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Parent {
    fn take_parent(&mut self) -> Option<TokenId>;
}
