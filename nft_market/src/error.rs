#[derive(Debug, thiserror::Error, near_sdk::FunctionError)]
pub enum ContractError {
    #[error("Contract's account id which call market's method are not supported")]
    NotAuthorized,
}
