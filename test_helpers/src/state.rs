use crate::HelperError;
use std::collections::HashMap;
use workspaces::{Account, Contract, DevNetwork, Worker};
pub type Accounts = HashMap<String, Account>;
pub type Contracts = HashMap<String, Contract>;

pub struct State<T> {
    root: Account,
    worker: Worker<T>,
    accounts: Accounts,
    contracts: Contracts,
}

impl<T: DevNetwork> State<T> {
    pub(crate) fn new(
        root: Account,
        worker: Worker<T>,
        accounts: Accounts,
        contracts: Contracts,
    ) -> Self {
        Self {
            root,
            worker,
            accounts,
            contracts,
        }
    }

    pub fn worker(&self) -> &Worker<T> {
        &self.worker
    }

    pub fn root(&self) -> &Account {
        &self.root
    }

    pub fn account(&self, id: impl AsRef<str>) -> Result<&Account, HelperError> {
        self.accounts
            .get(id.as_ref())
            .ok_or_else(|| HelperError::AccountNotFound(id.as_ref().to_owned()))
    }
    pub fn contract(&self, id: impl AsRef<str>) -> Result<&Contract, HelperError> {
        self.contracts
            .get(id.as_ref())
            .ok_or_else(|| HelperError::ContractNotFound(id.as_ref().to_owned()))
    }

    pub fn alice(&self) -> Result<&Account, HelperError> {
        self.account("alice")
    }

    pub fn bob(&self) -> Result<&Account, HelperError> {
        self.account("bob")
    }
}
