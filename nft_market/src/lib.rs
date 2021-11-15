use near_contract_standards::non_fungible_token::{
    approval::NonFungibleTokenApprovalReceiver, TokenId,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::env::panic_str;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise, PromiseError, PromiseOrValue, PromiseResult, Timestamp,
};
use std::borrow::BorrowMut;

pub const NO_DEPOSIT: Balance = 0;
pub const ONE_YOCTO: Balance = 1;
pub const BUY_METHOD_TOTAL_GAS: Gas = Gas(40_000_000_000_000);
pub const NFT_TRANSFER_GAS: Gas = Gas(10_000_000_000_000);
pub const AFTER_NFT_TRANSFER_GAS: Gas = Gas(5_000_000_000_000);

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    nft_id: AccountId,
    asks: UnorderedMap<TokenId, SaleCondition>,
    bids: UnorderedMap<TokenId, Vector<U128>>,
    trade_history: UnorderedMap<TokenId, Vector<Trade>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TradeType {
    Sell,
    Buy,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub prev_owner: AccountId,
    pub curr_owner: AccountId,
    pub price: U128,
    pub date: Timestamp,
    pub type_: TradeType,
}

impl Trade {
    pub fn from_sale(sale: SaleCondition, curr_owner: AccountId, type_: TradeType) -> Self {
        Self {
            prev_owner: sale.owner_id,
            curr_owner,
            price: sale.price,
            date: env::block_timestamp(),
            type_,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleCondition {
    pub owner_id: AccountId,
    pub token_id: TokenId,
    approval_id: u64,
    pub price: U128,
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
    Orders,
    TradeHistory,
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
    fn after_nft_transfer(&mut self, sale: SaleCondition, buyer_id: AccountId);
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(nft_id: AccountId) -> Self {
        Self {
            nft_id,
            asks: UnorderedMap::new(StorageKey::Asks),
            bids: UnorderedMap::new(StorageKey::Bids),
            trade_history: UnorderedMap::new(StorageKey::TradeHistory),
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

        let deposit = env::attached_deposit();
        require!(
            deposit == sale.price.0,
            "attached deposit isn't equal to token's price."
        );

        let prepaid_gas = env::prepaid_gas();
        require!(
            prepaid_gas >= BUY_METHOD_TOTAL_GAS,
            format!("attached gas less than: {:?}", BUY_METHOD_TOTAL_GAS)
        );

        let buyer_id = env::predecessor_account_id();
        self.process_purchase(token_id, sale.price, buyer_id);
    }

    fn process_purchase(&mut self, token_id: TokenId, price: U128, buyer_id: AccountId) -> Promise {
        let sale = self.asks.get(&token_id).unwrap();

        ext_nft::nft_transfer(
            buyer_id.clone(),
            token_id,
            Some(sale.approval_id),
            None,
            self.nft_id.clone(),
            ONE_YOCTO,
            NFT_TRANSFER_GAS,
        )
        .then(ext_self::after_nft_transfer(
            sale,
            buyer_id,
            env::current_account_id(),
            NO_DEPOSIT,
            AFTER_NFT_TRANSFER_GAS,
        ))
    }

    #[private]
    pub fn after_nft_transfer(&mut self, sale: SaleCondition, buyer_id: AccountId) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.asks.remove(&sale.token_id);
                self.add_trade_history(sale.clone(), buyer_id);
                Promise::new(sale.owner_id).transfer(sale.price.0);
            }
            PromiseResult::Failed => panic_str("Execution `nft_transfer` method was failed."),
            PromiseResult::NotReady => unreachable!(),
        }
    }

    fn add_trade_history(&mut self, sale: SaleCondition, buyer_id: AccountId) {
        let trade = Trade::from_sale(sale.clone(), buyer_id, TradeType::Sell);
        let mut history = self
            .trade_history
            .get(&sale.token_id)
            .unwrap_or_else(|| Vector::new(StorageKey::Orders));
        history.push(&trade);
        self.trade_history.insert(&sale.token_id, &history);
    }

    pub fn list_trade_history_by_token_id(&self, token_id: TokenId) -> Vec<Trade> {
        self.trade_history
            .get(&token_id)
            .map_or(Vec::new(), |v| v.to_vec())
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
        require!(env::predecessor_account_id() == self.nft_id);
        let SaleArgs { price } = serde_json::from_str(&msg).expect("couldn't parse json");
        let sale_conditions = SaleCondition::new(owner_id, token_id.clone(), approval_id, price);
        self.asks.insert(&token_id, &sale_conditions);
        let ret = json!({
            "status": true,
            "message": format!("token {} with price {} was added to market", token_id, price.0)
        });
        PromiseOrValue::Value(ret.to_string())
    }
}
