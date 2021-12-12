mod lemon;
mod slots;
mod weapon;

use enum_dispatch::enum_dispatch;
pub use lemon::Lemon;
use near_contract_standards::non_fungible_token::TokenId;
pub use slots::Slots;
pub use weapon::Weapon;

#[enum_dispatch(Slots)]
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
