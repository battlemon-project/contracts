use crate::{HelperError, State};
use anyhow::Context;
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use workspaces::network::DevAccountDeployer;
use workspaces::types::Balance;
use workspaces::{Account, DevNetwork, Worker};

pub struct StateBuilder<F> {
    worker_fut: fn() -> F,
    accounts: HashMap<String, Balance>,
    contracts: HashMap<String, (PathBuf, Balance)>,
}

impl<F, T> StateBuilder<F>
where
    F: Future<Output = anyhow::Result<Worker<T>>>,
    T: DevNetwork,
{
    pub fn new(worker_fut: fn() -> F) -> Self {
        Self {
            worker_fut,
            accounts: HashMap::new(),
            contracts: HashMap::new(),
        }
    }

    pub fn with_contract(
        mut self,
        id: &str,
        path: impl AsRef<std::path::Path>,
        balance: Balance,
    ) -> Result<Self, HelperError> {
        self.contracts
            .try_insert(id.to_owned(), (path.as_ref().to_path_buf(), balance))
            .map_err(|e| {
                HelperError::BuilderError(format!(
                    "Couldn't add task for contract creating with id `{}`",
                    e.entry.key()
                ))
            })?;

        Ok(self)
    }

    pub fn with_account<S: AsRef<str>>(
        mut self,
        id: S,
        balance: Balance,
    ) -> Result<Self, HelperError> {
        self.accounts
            .try_insert(id.as_ref().to_owned(), balance)
            .map_err(|e| {
                HelperError::BuilderError(format!(
                    "Couldn't add task for account creating with id `{}`",
                    e.entry.key()
                ))
            })?;
        Ok(self)
    }

    pub fn with_alice(self, balance: u128) -> Result<Self, HelperError> {
        self.with_account("alice", balance)
    }

    pub fn with_bob(self, balance: u128) -> Result<Self, HelperError> {
        self.with_account("bob", balance)
    }

    pub async fn build(self) -> Result<State<T>, HelperError> {
        let worker = (self.worker_fut)().await?;

        let root = worker
            .dev_create_account()
            .await
            .context("Failed to create root account while building")?;

        let (accounts, contracts) = self.process_accounts(&worker, &root).await?;

        Ok(State::new(root, worker, accounts, contracts))
    }

    async fn process_accounts(
        self,
        worker: &Worker<T>,
        root: &Account,
    ) -> Result<(crate::Accounts, crate::Contracts), HelperError> {
        let mut accounts_buf = HashMap::new();
        let mut contracts_buf = HashMap::new();

        let accounts = self
            .accounts
            .iter()
            .chain(self.contracts.iter().map(|(k, v)| (k, &v.1)));

        for (id, balance) in accounts {
            let account = root
                .create_subaccount(worker, id)
                .initial_balance(*balance)
                .transact()
                .await?
                .into_result()?;

            if let Some((path, _)) = self.contracts.get(id) {
                let wasm = tokio::fs::read(path).await.map_err(|e| {
                    HelperError::BuilderError(format!(
                        "Failed to read contract bytes from file {e}",
                    ))
                })?;

                let contract = account.deploy(worker, &wasm).await?.into_result()?;
                contracts_buf.insert(id.to_owned(), contract);
                continue;
            }
            accounts_buf.insert(id.to_owned(), account);
        }

        Ok((accounts_buf, contracts_buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NFT, NFT_PATH};

    #[test]
    fn builder_path_works() {
        StateBuilder::new(workspaces::testnet)
            .with_contract(NFT, NFT_PATH, 10)
            .unwrap();
    }

    #[test]
    fn builder_path_buf_works() {
        StateBuilder::new(workspaces::testnet)
            .with_contract(NFT, PathBuf::from(NFT_PATH), 10)
            .unwrap();
    }

    #[test]
    fn builder_ref_on_path_buf_works() {
        StateBuilder::new(workspaces::testnet)
            .with_contract(NFT, &PathBuf::from(NFT_PATH), 10)
            .unwrap();
    }

    #[test]
    fn builder_account_str_works() {
        StateBuilder::new(workspaces::testnet)
            .with_account("alice", 10)
            .unwrap();
    }

    #[test]
    fn builder_account_string_works() {
        StateBuilder::new(workspaces::testnet)
            .with_account(String::from("alice"), 10)
            .unwrap();
    }

    #[test]
    fn builder_account_ref_on_string_works() {
        StateBuilder::new(workspaces::testnet)
            .with_account(&String::from("alice"), 10)
            .unwrap();
    }
}
