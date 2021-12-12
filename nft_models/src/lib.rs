mod lemon;
mod slots;

use enum_dispatch::enum_dispatch;
pub use lemon::Lemon;
use near_contract_standards::non_fungible_token::TokenId;
pub use slots::Slots;

#[enum_dispatch(Slots)]
pub enum ModelKind {
    Lemon,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
