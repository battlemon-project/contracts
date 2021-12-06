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

enum OrderType {
    AcceptAsk,
    AskLessBid(AccountId),
    AcceptBid {
        owner_id: AccountId,
        approval_id: u64,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
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
#[serde(rename_all = "snake_case")]
pub enum SaleType {
    AcceptBid,
    Selling,
}

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleArgs {
    sale_type: SaleType,
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

    fn nft_token(&self, token_id: TokenId) -> Promise;
}

#[near_sdk::ext_contract]
trait ExtSelf {
    fn after_nft_transfer_for_ask(&mut self, sale: SaleCondition, buyer_id: AccountId) -> Promise;
    fn after_nft_transfer_for_bid(
        &mut self,
        sale: OfferCondition,
        owner_id: AccountId,
    ) -> PromiseOrValue<()>;

    fn after_nft_token(
        &mut self,
        bidder_id: AccountId,
        token_id: TokenId,
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
        self.process_purchase(token_id, OrderType::Ask)
    }

    #[payable]
    pub fn bid(&mut self, token_id: TokenId) -> Promise {
        let deposit = env::attached_deposit();
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
            env::current_account_id(),
            deposit,
            Gas(90_000_000_000_000),
        ))
    }

    #[payable]
    pub fn accept_bid(&mut self, token_id: TokenId) -> Promise {
        self.process_purchase(token_id, OrderType::Bid)
    }

    pub fn list_bids(&self) -> Vec<(TokenId, Vec<OfferCondition>)> {
        self.bids.to_vec()
    }

    fn process_purchase(&mut self, token_id: TokenId, order_type: OrderType) -> Promise {
        let approval_id;
        let callback;
        let new_owner_id;

        match order_type {
            OrderType::Ask => {
                new_owner_id = env::predecessor_account_id();

                let sale = self.get_ask(&token_id);

                // todo: to refactor from here ->
                let deposit = env::attached_deposit();
                let panic_msg = format!(
                    "attached deposit less than token's price.\n\
                     Attached deposit is {}, token's price is {}",
                    deposit, sale.price.0
                );
                require!(deposit >= sale.price.0, panic_msg);

                require!(
                    env::prepaid_gas() >= BUY_METHOD_TOTAL_GAS,
                    format!("attached gas less than: {:?}", BUY_METHOD_TOTAL_GAS)
                );
                // <- until here

                approval_id = Some(sale.approval_id);

                callback = ext_self::after_nft_transfer_for_ask(
                    sale,
                    new_owner_id.clone(),
                    env::current_account_id(),
                    deposit,
                    AFTER_NFT_TRANSFER_GAS,
                );
            }
            OrderType::AskLessBid(bidder_id) => {
                let sale = self.get_ask(&token_id);

                // todo: to refactor from here ->
                let deposit = env::attached_deposit();
                let panic_msg = format!(
                    "attached deposit less than token's price.\n\
                     Attached deposit is {}, token's price is {}",
                    deposit, sale.price.0
                );
                require!(deposit >= sale.price.0, panic_msg);

                require!(
                    env::prepaid_gas() >= BUY_METHOD_TOTAL_GAS,
                    format!("attached gas less than: {:?}", BUY_METHOD_TOTAL_GAS)
                );
                // <- until here
                new_owner_id = bidder_id.clone();
                approval_id = Some(sale.approval_id);

                callback = ext_self::after_nft_transfer_for_ask(
                    sale,
                    bidder_id,
                    env::current_account_id(),
                    deposit,
                    AFTER_NFT_TRANSFER_GAS,
                );
            }

            OrderType::Bid => {
                let deposit = env::attached_deposit();
                let mut ordered_bids = self.bids.get(&token_id).unwrap_or_else(|| {
                    panic_str(format!("bids for token is: {}, doesn't exists", token_id).as_str())
                });

                ordered_bids.sort_by_key(|v| v.price.0);

                let last = ordered_bids
                    .last()
                    .unwrap_or_else(|| panic_str("bids is empty"));

                require!(
                    deposit == last.price.0,
                    "attached deposit must be equal to bid price"
                );

                new_owner_id = last.bidder_id.clone();
                approval_id = None;
                callback = ext_self::after_nft_transfer_for_bid(
                    last.clone(),
                    env::predecessor_account_id(),
                    env::current_account_id(),
                    env::attached_deposit(),
                    AFTER_NFT_TRANSFER_GAS,
                )
            }
        };

        ext_nft::nft_transfer(
            new_owner_id,
            token_id,
            approval_id,
            None,
            self.nft_id.clone(),
            ONE_YOCTO,
            NFT_TRANSFER_GAS,
        )
        .then(callback)
    }

    fn get_ask(&self, token_id: &TokenId) -> SaleCondition {
        self.asks.get(token_id).unwrap_or_else(|| {
            panic_str(format!("token with id {} doesn't sell", token_id).as_str())
        })
    }

    #[private]
    #[payable]
    pub fn after_nft_transfer_for_ask(
        &mut self,
        sale: SaleCondition,
        buyer_id: AccountId,
    ) -> Promise {
        let deposit = env::attached_deposit();
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.asks.remove(&sale.token_id);
                self.add_trade_history(sale.clone(), buyer_id.clone());

                let trade = Promise::new(sale.owner_id).transfer(sale.price.0);
                let change = Promise::new(buyer_id).transfer(deposit - sale.price.0);

                if deposit > sale.price.0 {
                    log!("bid more than sale price, refund change and paying for sale token");
                    trade.then(change)
                } else {
                    log!("bid equals to sale price, just paying for sale token");
                    trade
                }
            }
            PromiseResult::Failed => {
                log!("Execution `nft_transfer` method was failed. Attached deposit was refund.");
                Promise::new(buyer_id).transfer(deposit)
            }
            PromiseResult::NotReady => unreachable!(),
        }
    }

    #[private]
    #[payable]
    pub fn after_nft_transfer_for_bid(
        &mut self,
        sale: SaleCondition,
        buyer_id: AccountId,
    ) -> Promise {
        let deposit = env::attached_deposit();
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                // self.asks.remove(&sale.token_id);
                // self.add_trade_history(sale.clone(), buyer_id.clone());
                //
                // let trade = Promise::new(sale.owner_id).transfer(sale.price.0);
                // let change = Promise::new(buyer_id).transfer(deposit - sale.price.0);
                //
                // if deposit > sale.price.0 {
                //     log!("bid more than sale price, refund change and paying for sale token");
                //     trade.then(change)
                // } else {
                //     log!("bid more equals to sale price, just paying for sale token");
                //     trade
                // }
                todo!()
            }
            PromiseResult::Failed => {
                log!("Execution `nft_transfer` method was failed. Attached deposit was refund.");
                Promise::new(buyer_id).transfer(deposit)
            }
            PromiseResult::NotReady => unreachable!(),
        }
    }

    #[private]
    #[payable]
    pub fn after_nft_token(
        &mut self,
        bidder_id: AccountId,
        token_id: TokenId,
        #[rustfmt::skip]
        #[callback_result]
        result: Result<Option<TokenExt>, PromiseError>,
    ) -> PromiseOrValue<()> {
        let bid_price = env::attached_deposit();
        let ask_less_bid = self
            .asks
            .get(&token_id)
            .map_or(false, |ask| ask.price.0 <= bid_price);

        match result {
            Ok(Some(_)) if ask_less_bid => {
                log!("ask for current token id less than provided bid, so process purchase");
                self.process_purchase(token_id, OrderType::AskLessBid(bidder_id));
                PromiseOrValue::Value(())
            }
            Ok(Some(_)) => {
                let new_offer_condition =
                    OfferCondition::new(token_id.clone(), bidder_id, bid_price);

                match self.bids.get(&token_id).as_mut() {
                    Some(offer_conditions) => {
                        offer_conditions.push(new_offer_condition);
                        self.bids.insert(&token_id, offer_conditions)
                    }
                    _ => self.bids.insert(&token_id, &vec![new_offer_condition]),
                };

                PromiseOrValue::Value(())
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{serde_json, testing_env};
    use test_utils::*;

    use super::*;

    #[test]
    fn init() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::init(accounts(1));
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.list_asks().len(), 0);
        assert_eq!(contract.list_bids().len(), 0);
    }

    #[test]
    #[should_panic(expected = "token with id valid token id doesn't sell")]
    fn get_ask_empty_asks() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::init(accounts(1));
        testing_env!(context.is_view(true).build());
        contract.get_ask(&VALID_TOKEN_ID.to_string());
    }

    #[test]
    #[should_panic(expected = "token with id invalid token id doesn't sell")]
    fn get_ask_from_not_empty_asks_but_invalid_token_id() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(1));
        let ask = SaleCondition::new(accounts(2), VALID_TOKEN_ID.to_string(), 0, 0);
        contract.asks.insert(&VALID_TOKEN_ID.to_string(), &ask);
        testing_env!(context.is_view(true).build());
        contract.get_ask(&INVALID_TOKEN_ID.to_string());
    }

    #[test]
    fn get_ask_valid_token_id() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(1));
        let expected_ask = SaleCondition::new(accounts(2), VALID_TOKEN_ID.to_string(), 0, 0);
        contract
            .asks
            .insert(&VALID_TOKEN_ID.to_string(), &expected_ask);
        testing_env!(context.is_view(true).build());
        let actual_ask = contract.get_ask(&VALID_TOKEN_ID.to_string());
        assert_eq!(expected_ask, actual_ask);
    }
}
