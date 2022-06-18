#![feature(map_try_insert)]

mod consts;
mod errors;

use anyhow::Context;
use std::collections::HashMap;

use errors::HelperError;
use workspaces::{Account, Contract};

pub use consts::*;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::AccountId;
pub use near_units::{parse_gas, parse_near};
use nft_models::lemon::Lemon;
use nft_models::ModelKind;
use token_metadata_ext::*;
pub use workspaces;
use workspaces::network::{
    AllowDevAccountCreation, DevAccountDeployer, NetworkClient, NetworkInfo, TopLevelAccountCreator,
};
use workspaces::types::Balance;
use workspaces::Worker;

pub fn alice() -> AccountId {
    AccountId::new_unchecked("alice.near".to_string())
}

pub fn bob() -> AccountId {
    AccountId::new_unchecked("bob.near".to_string())
}
pub fn danny() -> AccountId {
    AccountId::new_unchecked("danny.near".to_string())
}
pub fn fargo() -> AccountId {
    AccountId::new_unchecked("fargo.near".to_string())
}

pub fn carol() -> AccountId {
    AccountId::new_unchecked("carol.near".to_string())
}

pub fn tokens<const N: usize>() -> [TokenId; N] {
    let range: Vec<_> = (0..N).map(|v| v.to_string()).collect();
    <[_; N]>::try_from(range).unwrap()
}

pub struct State<'a, T> {
    root: Account,
    worker: Worker<T>,
    accounts: HashMap<&'a str, Account>,
    contracts: HashMap<&'a str, Contract>,
}

impl<'a, T> State<'a, T> {
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

        Ok(State {
            root,
            worker: self.worker,
            accounts: accounts_buf,
            contracts: contracts_buf,
        })
    }
}
// pub fn fake_metadata_with<T>(model: T) -> TokenMetadataExt
// where
//     T: Manager + Into<ModelKind>,
// {
//     TokenMetadataExt {
//         title: Some("fake title".into()),
//         description: Some("this is fake description".into()),
//         media: Some("https://fakelinktomedia.com".into()),
//         media_hash: Some(vec![0, 1, 2, 3, 4].into()),
//         copies: None,
//         issued_at: None,
//         expires_at: None,
//         starts_at: None,
//         updated_at: None,
//         extra: None,
//         reference: None,
//         reference_hash: None,
//         model: model.into(),
//     }
// }

pub fn sample_token_metadata() -> TokenMetadataExt {
    let model: ModelKind = ModelKind::Lemon(get_foo_lemon());

    TokenMetadataExt {
        title: Some("foo title".into()),
        description: Some("this is description for foo title's token".into()),
        media: None,
        media_hash: None,
        copies: Some(1),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
        model,
    }
}

pub fn get_foo_lemon() -> Lemon {
    use nft_models::lemon::*;

    Lemon::from_random(&[10, 11, 12, 14])
}

// pub fn get_foo_weapon() -> Weapon {
//     use nft_models::weapon::*;
//
//     Weapon {
//         level: 0,
//         r#type: Type::Instant,
//         parent: None,
//         slots: HashSet::new(),
//     }
// }
//
// pub fn foo_token_metadata_ext() -> TokenMetadataExt {
//     use nft_models::weapon::*;
//     let model: ModelKind = Weapon {
//         level: 0,
//         r#type: Type::Instant,
//         parent: None,
//         slots: HashSet::new(),
//     }
//     .into();
//
//     TokenMetadataExt {
//         title: Some("foo_token".into()),
//         description: Some("this is description for foo title's token".into()),
//         media: Some("link to media".into()),
//         media_hash: Some(vec![0, 1, 2, 3, 4].into()),
//         copies: Some(1),
//         issued_at: None,
//         expires_at: None,
//         starts_at: None,
//         updated_at: None,
//         extra: Some(
//             json!({
//                 "some": "json",
//                 "values": 9,
//             })
//             .to_string(),
//         ),
//         reference: None,
//         reference_hash: None,
//         model,
//     }
// }
//
// pub fn baz_token_metadata_ext() -> TokenMetadataExt {
//     use nft_models::lemon::*;
//     let model: ModelKind = Lemon {
//         option: Exo::MA01,
//         cap: Cap::Otherworldly,
//         r#type: Cloth::Heavy,
//         eyes: Eyes::Nakamoto,
//         background: Head::Red,
//         top: Teeth::Classical,
//         cyber_suit: CyberSuit::Black,
//         expression: Expression::Angry,
//         eyes: Eyes::Close,
//         hair: Hair::Punkkez,
//         accessory: Accessory::Toothpick,
//         winrate: Some(33),
//         rarity: 88,
//         teeth: None,
//         slots: HashSet::new(),
//     }
//     .into();
//
//     TokenMetadataExt {
//         title: Some("baz_token".into()),
//         description: Some("this is description for baz title's token".into()),
//         media: Some("link to media".into()),
//         media_hash: Some(vec![2, 3, 4, 3, 4].into()),
//         copies: Some(1),
//         issued_at: None,
//         expires_at: None,
//         starts_at: None,
//         updated_at: None,
//         extra: Some(
//             json!({
//                 "rust": "bless you!",
//                 "values": 2,
//             })
//             .to_string(),
//         ),
//         reference: None,
//         reference_hash: None,
//         model,
//     }
// }

pub fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(alice())
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

pub async fn deploy_contract<T: workspaces::Network>(
    worker: &Worker<T>,
    account_id: &str,
    deposit: Balance,
    root: &Account,
    path: impl AsRef<std::path::Path>,
) -> anyhow::Result<Contract> {
    let wasm = tokio::fs::read(path)
        .await
        .with_context(|| format!("Failed to load market wasm for {account_id}"))?;

    let account = root
        .create_subaccount(worker, account_id)
        .initial_balance(deposit)
        .transact()
        .await
        .with_context(|| format!("Failed to create sub-account for {account_id}"))?
        .into_result()?;

    account
        .deploy(worker, &wasm)
        .await
        .with_context(|| format!("Failed to deploy contract for {account_id}"))?
        .into_result()
}
