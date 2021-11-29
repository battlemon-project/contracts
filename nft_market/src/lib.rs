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

use token_metadata_ext::TokenExt;

pub const NO_DEPOSIT: Balance = 0;
pub const ONE_YOCTO: Balance = 1;
pub const BUY_METHOD_TOTAL_GAS: Gas = Gas(80_000_000_000_000);
pub const NFT_TRANSFER_GAS: Gas = Gas(44_000_000_000_000);
pub const AFTER_NFT_TRANSFER_GAS: Gas = Gas(20_000_000_000_000);

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    nft_id: AccountId,
    asks: UnorderedMap<TokenId, SaleCondition>,
    bids: UnorderedMap<TokenId, Vec<OfferCondition>>,
    trade_history: UnorderedMap<TokenId, Vector<Trade>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum TradeType {
    Sell,
    Buy,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub prev_owner: AccountId,
    pub curr_owner: AccountId,
    pub price: u128,
    pub date: Timestamp,
    #[serde(rename = "type")]
    pub type_: TradeType,
}

impl Trade {
    pub fn from_sale(sale: SaleCondition, curr_owner: AccountId, type_: TradeType) -> Self {
        Self {
            prev_owner: sale.owner_id,
            curr_owner,
            price: sale.price.0,
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
    pub fn new(owner_id: AccountId, token_id: TokenId, approval_id: u64, price: u128) -> Self {
        Self {
            owner_id,
            token_id,
            approval_id,
            price: U128(price),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferCondition {
    pub token_id: TokenId,
    pub bidder_id: AccountId,
    pub price: U128,
}

impl OfferCondition {
    pub fn new(token_id: TokenId, bidder_id: AccountId, price: u128) -> Self {
        Self {
            token_id,
            bidder_id,
            price: U128(price),
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
    ) -> Promise;

    // fn nft_token(&self, token_id: TokenId) -> Option<TokenExt>;
    fn nft_token(&self, token_id: TokenId) -> Promise;
}

#[near_sdk::ext_contract]
trait ExtSelf {
    fn after_nft_transfer(&mut self, sale: SaleCondition, buyer_id: AccountId) -> Promise;

    fn after_nft_token(
        &mut self,
        bidder_id: AccountId,
        token_id: TokenId,
        bid_price: Balance,
        #[rustfmt::skip]
        #[callback_result]
        result: Result<Option<TokenExt>, PromiseError>,
    ) -> PromiseOrValue<()>;
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
    pub fn buy(&mut self, token_id: TokenId) -> Promise {
        let sale = self.asks.get(&token_id).unwrap_or_else(|| {
            panic_str(format!("token with id {} doesn't sell", token_id).as_str())
        });

        let deposit = env::attached_deposit();
        require!(
            deposit == sale.price,
            format!(
                "attached deposit isn't equal to token's price.\n attached deposit is {}, token's price is {}",
                deposit, sale.price
            )
        );

        let prepaid_gas = env::prepaid_gas();
        require!(
            prepaid_gas >= BUY_METHOD_TOTAL_GAS,
            format!("attached gas less than: {:?}", BUY_METHOD_TOTAL_GAS)
        );

        let buyer_id = env::predecessor_account_id();
        self.process_purchase(token_id, buyer_id)
    }

    #[payable]
    pub fn bid(&mut self, token_id: TokenId) -> Promise {
        let bid_price = env::attached_deposit();
        let bidder_id = env::predecessor_account_id();

        ext_nft::nft_token(
            token_id.clone(),
            self.nft_id.clone(),
            NO_DEPOSIT,
            Gas(20_000_000_000_000),
        )
        .then(ext_self::after_nft_token(
            bidder_id,
            token_id,
            bid_price,
            env::current_account_id(),
            NO_DEPOSIT,
            Gas(20_000_000_000_000),
        ))
    }

    pub fn list_bids(&self) -> Vec<OfferCondition> {
        self.bids.values().collect()
    }

    fn process_purchase(&mut self, token_id: TokenId, buyer_id: AccountId) -> Promise {
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
    pub fn after_nft_transfer(&mut self, sale: SaleCondition, buyer_id: AccountId) -> Promise {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.asks.remove(&sale.token_id);
                self.add_trade_history(sale.clone(), buyer_id);
                Promise::new(sale.owner_id).transfer(sale.price)
            }
            PromiseResult::Failed => panic_str("Execution `nft_transfer` method was failed."),
            PromiseResult::NotReady => unreachable!(),
        }
    }

    #[private]
    pub fn after_nft_token(
        &mut self,
        bidder_id: AccountId,
        token_id: TokenId,
        bid_price: Balance,
        #[rustfmt::skip]
        #[callback_result]
        result: Result<Option<TokenExt>, PromiseError>,
    ) -> PromiseOrValue<()> {
        match result {
            Ok(Some(_)) => {
                let ask_less_bid = self
                    .asks
                    .get(&token_id)
                    .map_or(false, |ask| ask.price <= bid_price);

                if ask_less_bid {
                    todo!("nft token transfer");
                    todo!("return attached deposit - ask.price");
                    // return PromiseOrValue::Promise(_);
                }

                match self.bids.get(&token_id) {
                    Some(offer_condition) if bid_price <= offer_condition.price => {
                        log!(
                            "your bid is {} and it's less than last bid {}",
                            bid_price,
                            offer_condition.price
                        );
                        let promise = Promise::new(bidder_id).transfer(bid_price);
                        PromiseOrValue::Promise(promise)
                    }
                    _ => {
                        self.bids.insert(
                            &token_id,
                            &OfferCondition::new(token_id.clone(), bidder_id, bid_price),
                        );
                        PromiseOrValue::Value(())
                    }
                }
            }

            Ok(None) => {
                log!(
                    "token with id: {} doesn't exist, attached deposit was returned",
                    token_id
                );
                let promise = Promise::new(bidder_id).transfer(bid_price);
                PromiseOrValue::Promise(promise)
            }
            Err(_) => {
                log!("`nft_token` execution error was occurred, attached deposit was returned");
                let promise = Promise::new(bidder_id).transfer(bid_price);
                PromiseOrValue::Promise(promise)
            }
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
        let sale_conditions = SaleCondition::new(owner_id, token_id.clone(), approval_id, price.0);
        self.asks.insert(&token_id, &sale_conditions);
        let ret = json!({
            "status": true,
            "message": format!("token {} with price {} was added to market", token_id, price.0)
        });
        PromiseOrValue::Value(ret.to_string())
    }
}
