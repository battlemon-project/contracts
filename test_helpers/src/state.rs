use crate::HelperError;
use std::collections::HashMap;
use workspaces::{Account, Contract, Worker};
type Accounts<'a> = HashMap<&'a str, Account>;
type Contracts<'a> = HashMap<&'a str, Contract>;

pub struct State<'a, T> {
    root: Account,
    worker: Worker<T>,
    accounts: Accounts<'a>,
    contracts: Contracts<'a>,
}

impl<'a, T> State<'a, T> {
    pub fn new(
        root: Account,
        worker: Worker<T>,
        accounts: Accounts<'a>,
        contracts: Contracts<'a>,
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

    pub fn account(&self, id: &'a str) -> Result<&Account, HelperError> {
        self.accounts
            .get(id)
            .ok_or_else(|| HelperError::AccountNotFound(id.to_string()))
    }
    pub fn contract(&self, id: &'a str) -> Result<&Contract, HelperError> {
        self.contracts
            .get(id)
            .ok_or_else(|| HelperError::ContractNotFound(id.to_string()))
    }

    pub fn alice(&self) -> Result<&Account, HelperError> {
        self.account("alice")
    }

    pub fn bob(&self) -> Result<&Account, HelperError> {
        self.account("bob")
    }
}
