use std::error::Error;
use std::fmt::{self, Debug, Formatter};

#[derive(thiserror::Error)]
pub enum HelperError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Faild to build state: {0}")]
    BuilderError(String),
    #[error("Failed to get account with id {0}")]
    AccountNotFound(String),
    #[error("Failed to get contract with id {0}")]
    ContractNotFound(String),
}

impl Debug for HelperError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(error: &impl Error, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}\n", error)?;
    let mut current = error.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}
