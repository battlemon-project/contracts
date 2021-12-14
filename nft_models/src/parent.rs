use near_contract_standards::non_fungible_token::TokenId;

pub trait Parent {
    fn take_parent(self) -> TokenId;
}
