#[derive(Debug, thiserror::Error, near_sdk::FunctionError)]
pub enum ContractError {
    #[error("Contract's account id which call market's method are not supported")]
    NotAuthorized,
    #[error("Failed to add bid: {0}")]
    BidError(String),
    #[error(transparent)]
    SerdeError(#[from] near_sdk::serde_json::Error),
}
