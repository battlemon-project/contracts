use crate::consts::STORAGE_PER_SALE;
use std::fmt::{Debug, Display, Formatter};

#[derive(thiserror::Error, near_sdk::FunctionError)]
pub enum ContractError {
    #[error("Failed to authorize contract call: {0}")]
    NotAuthorized(&'static str),
    #[error("Failed to add bid: {0}")]
    BidError(String),
    #[error(transparent)]
    SerdeError(#[from] near_sdk::serde_json::Error),
    #[error("Requires minimum deposit of {STORAGE_PER_SALE}")]
    InsufficientDeposit,
    #[error("The attached deposit must be equal to 1 yoctoNear")]
    OneYoctoDeposit,
    #[error("The storage error occurred: {0}")]
    StorageError(&'static str),
    #[error("The ask does not exist")]
    AskNotFound,
}

impl Debug for ContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
