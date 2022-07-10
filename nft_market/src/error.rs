use crate::consts::STORAGE_PER_SALE;
use std::fmt::{Debug, Display, Formatter};

#[derive(thiserror::Error, near_sdk::FunctionError)]
pub enum ContractError {
    #[error("Contract's account id which call market's method are not supported")]
    NotAuthorized,
    #[error("Failed to add bid: {0}")]
    BidError(String),
    #[error(transparent)]
    SerdeError(#[from] near_sdk::serde_json::Error),
    #[error("Requires minimum deposit of {STORAGE_PER_SALE}")]
    InsufficientDeposit,
}

impl Debug for ContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
