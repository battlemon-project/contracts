use crate::{HelperError, State};
use anyhow::Context;
use std::collections::HashMap;
use workspaces::network::{
    AllowDevAccountCreation, DevAccountDeployer, NetworkClient, NetworkInfo, TopLevelAccountCreator,
};
use workspaces::types::Balance;
use workspaces::Worker;

pub struct StateBuilder<'a, T> {
    worker: Worker<T>,
    accounts: HashMap<&'a str, Balance>,
    contracts: HashMap<&'a str, (&'a str, Balance)>,
}

impl<T> StateBuilder<'static, T>
where
    T: TopLevelAccountCreator + NetworkInfo + AllowDevAccountCreation + Send + Sync + NetworkClient,
{
    pub fn new(worker: Worker<T>) -> Self {
        Self {
            worker,
            accounts: HashMap::new(),
            contracts: HashMap::new(),
        }
    }

    pub fn with_contract(
        mut self,
        name: &'static str,
        path: &'static str,
        balance: Balance,
    ) -> Result<Self, HelperError> {
        self.contracts
            .try_insert(name, (path, balance))
            .map_err(|e| {
                HelperError::BuilderError(format!(
                    "Couldn't add task for contract creating with id `{}`",
                    e.entry.key()
                ))
            })?;

        Ok(self)
    }

    pub fn with_account(
        mut self,
        name: &'static str,
        balance: Balance,
    ) -> Result<Self, HelperError> {
        self.accounts.try_insert(name, balance).map_err(|e| {
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

    pub async fn build(self) -> Result<State<'static, T>, HelperError> {
        let root = self
            .worker
            .dev_create_account()
            .await
            .context("Failed to create root account while building")?;

        let accounts = self
            .accounts
            .iter()
            .chain(self.contracts.iter().map(|(k, v)| (k, &v.1)));

        let mut accounts_buf = HashMap::new();
        let mut contracts_buf = HashMap::new();

        for (id, balance) in accounts {
            let account = root
                .create_subaccount(&self.worker, id)
                .initial_balance(*balance)
                .transact()
                .await?
                .into_result()?;

            if let Some((path, balance)) = self.contracts.get(id) {
                let wasm = tokio::fs::read(path).await.map_err(|e| {
                    HelperError::BuilderError(format!(
                        "Failed to read contract bytes from file {e}",
                    ))
                })?;

                let contract = account.deploy(&self.worker, &wasm).await?.into_result()?;
                contracts_buf.insert(*id, contract);
                continue;
            }
            accounts_buf.insert(*id, account);
        }

        Ok(State::new(root, self.worker, accounts_buf, contracts_buf))
    }
}
