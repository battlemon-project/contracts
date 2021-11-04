use near_contract_standards::non_fungible_token::{
    approval::NonFungibleTokenApprovalReceiver, TokenId,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::env::panic_str;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};

const NO_DEPOSIT: Balance = 0;
const XCC_GAS: Gas = Gas(200_000_000_000_000);

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    nft_id: AccountId,
    asks: UnorderedMap<TokenId, SaleCondition>,
    bids: UnorderedMap<TokenId, Vector<U128>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleCondition {
    owner_id: AccountId,
    token_id: TokenId,
    approval_id: u64,
    price: U128,
}

impl SaleCondition {
    pub fn new(owner_id: AccountId, token_id: TokenId, approval_id: u64, price: U128) -> Self {
        Self {
            owner_id,
            token_id,
            approval_id,
            price,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    price: U128,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Asks,
    Bids,
}

#[near_sdk::ext_contract]
trait ExtNft {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}

#[near_sdk::ext_contract(ext_self)]
trait ExtSelf {
    fn after_nft_transfer(&mut self, sale: SaleCondition);
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(nft_id: AccountId) -> Self {
        Self {
            nft_id,
            asks: UnorderedMap::new(StorageKey::Asks),
            bids: UnorderedMap::new(StorageKey::Bids),
        }
    }

    pub fn list_asks(&self) -> Vec<SaleCondition> {
        self.asks.iter().map(|(_, v)| v).collect()
    }

    #[payable]
    pub fn buy(&mut self, token_id: TokenId) {
        let sale = self.asks.get(&token_id).unwrap_or_else(|| {
            panic_str(format!("token with id {} doesn't sell", token_id).as_str())
        });

        let buyer_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        require!(deposit > 0, "attached deposit must be more than 0");
        // require!(
        //     deposit == sale.price.0,
        //     "attached deposit isn't equal to token's price."
        // );

        self.process_purchase(token_id, sale.price, buyer_id);
    }

    #[private]
    pub fn process_purchase(
        &mut self,
        token_id: TokenId,
        price: U128,
        buyer_id: AccountId,
    ) -> Promise {
        let sale = self.asks.get(&token_id).unwrap();

        ext_nft::nft_transfer(
            buyer_id,
            token_id,
            Some(sale.approval_id),
            None,
            self.nft_id.clone(),
            1,
            XCC_GAS,
        )
        .then(ext_self::after_nft_transfer(
            sale,
            env::current_account_id(),
            0,
            Gas(10_000_000_000_000),
        ))
    }

    #[private]
    pub fn after_nft_transfer(&mut self, sale: SaleCondition) {
        require!(
            env::promise_results_count() > 0,
            "doesn't have result of cross-contract call"
        );

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                self.asks.remove(&sale.token_id);
                env::log_str("we are here!!!")
                // Promise::new(env::current_account_id()).transfer(sale.price.0);
            }
            PromiseResult::Failed => panic_str("nft_transfer was failed"),
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> PromiseOrValue<String> {
        // TODO: security assertions
        // require!(env::predecessor_account_id() == self.nft_id);
        let SaleArgs { price } = serde_json::from_str(&msg).expect("couldn't parse json");
        let sale_conditions = SaleCondition::new(owner_id, token_id.clone(), approval_id, price);
        self.asks.insert(&token_id, &sale_conditions);

        PromiseOrValue::Value(format!(
            "token {} with price {} was added to market",
            token_id, price.0
        ))
    }
}