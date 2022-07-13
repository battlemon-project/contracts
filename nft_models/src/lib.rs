use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

pub use lemon::Lemon;

pub mod lemon;

#[derive(
    Serialize, Deserialize, Clone, Copy, BorshSerialize, BorshDeserialize, Debug, PartialEq,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case", tag = "kind")]
pub enum ModelKind {
    Lemon(Lemon),
}

impl BuildUrlQuery for ModelKind {
    fn build_url_query(&self) -> String {
        match self {
            Self::Lemon(lemon) => lemon.build_url_query(),
        }
    }
}

pub trait BuildUrlQuery {
    fn build_url_query(&self) -> String;
}
// impl ModelKind {
//     pub fn is_compatible(&self, model_kind: &Self) -> bool {
//         match (self, model_kind) {
//             (ModelKind::Lemon(_), ModelKind::Weapon(_)) => true,
//             (ModelKind::Weapon(_), ModelKind::Suppressor(_)) => true,
//             _ => false,
//         }
//     }
// }
