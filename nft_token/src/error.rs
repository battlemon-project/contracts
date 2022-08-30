use near_contract_standards::non_fungible_token::TokenId;

#[derive(Debug, thiserror::Error, near_sdk::FunctionError)]
pub enum ContractError {
    #[error("Failed to authorize contract call: {0}")]
    NotAuthorized(&'static str),
    #[error("Failed to find model for token with id: {0}")]
    ModelNotFound(TokenId),
    #[error("Failed to find owner for token with id: {0}")]
    OwnerNotFound(TokenId),
    #[error("Provided instructions contain errors because: {0}")]
    InstructionError(String),
    #[error(transparent)]
    SerdeError(#[from] near_sdk::serde_json::Error),
}

pub(crate) type Result<T> = std::result::Result<T, ContractError>;
