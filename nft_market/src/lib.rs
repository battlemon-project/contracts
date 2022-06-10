use ask::*;
use bid::*;
use callback::*;
use consts::*;
use domain::*;
use error::*;
use internal::*;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::Deserialize;
use near_sdk::serde_json::{self, json};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseError, PromiseOrValue, PromiseResult,
};
use trade::*;

mod ask;
mod bid;
mod callback;
mod consts;
mod domain;
mod error;
mod external;
mod internal;
mod offer;
mod order;
mod trade;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    nft_id: AccountId,
    asks: UnorderedMap<TokenId, Ask>,
    bids: UnorderedMap<TokenId, Vec<Bid>>,
}

enum OrderType {
    AcceptAsk,
    AskLessBid(AccountId),
    AcceptBid {
        owner_id: AccountId,
        approval_id: u64,
    },
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
    price: Option<U128>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Asks,
    Bids,
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

    pub fn bid(&mut self, bid: Bid) {
        self.add_bid(bid);
    }
}

#[cfg(all(not(target_arch = "wasm32"), test))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn contract_new_works() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::init(accounts(1));
        assert_eq!(contract.nft_id, accounts(1));
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn uninitialized_contract_must_panic() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        Contract::default();
    }
}
// pub fn list_asks(&self) -> Vec<Ask> {
//     self.asks.iter().map(|(_, v)| v).collect()
// }
//
// #[payable]
// pub fn buy(&mut self, token_id: TokenId) -> Promise {
//     self.process_purchase(token_id, OrderType::AcceptAsk)
// }
//
// #[payable]
// pub fn bid(&mut self, token_id: TokenId) -> Promise {
//     let deposit = env::attached_deposit();
//     require!(deposit > 0, "attached deposit must be more than 0.");
//
//     let bidder_id = env::predecessor_account_id();
//
//     ext_nft::nft_token(
//         token_id.clone(),
//         self.nft_id.clone(),
//         NO_DEPOSIT,
//         Gas(20_000_000_000_000),
//     )
//     .then(ext_self::after_nft_token(
//         bidder_id,
//         token_id,
//         env::current_account_id(),
//         deposit,
//         Gas(90_000_000_000_000),
//     ))
// }
//
// pub fn list_bids(&self) -> Vec<(TokenId, Vec<Bid>)> {
//     self.bids.to_vec()
// }
//
// fn process_purchase(&mut self, token_id: TokenId, order_type: OrderType) -> Promise {
//     let approval_id;
//     let callback;
//     let new_owner_id;
//
//     match order_type {
//         OrderType::AcceptAsk => {
//             new_owner_id = env::predecessor_account_id();
//
//             let sale = self.get_ask(&token_id);
//
//             // todo: to refactor from here ->
//             let deposit = env::attached_deposit();
//             let panic_msg = format!(
//                 "attached deposit less than token's price.\n\
//                  Attached deposit is {}, token's price is {}",
//                 deposit, sale.price.0
//             );
//             require!(deposit >= sale.price.0, panic_msg);
//
//             require!(
//                 env::prepaid_gas() >= BUY_METHOD_TOTAL_GAS,
//                 format!("attached gas less than: {:?}", BUY_METHOD_TOTAL_GAS)
//             );
//             // <- until here
//
//             approval_id = sale.approval_id;
//
//             callback = ext_self::after_nft_transfer_for_ask(
//                 sale,
//                 new_owner_id.clone(),
//                 env::current_account_id(),
//                 deposit,
//                 AFTER_NFT_TRANSFER_GAS,
//             );
//         }
//         OrderType::AskLessBid(bidder_id) => {
//             let sale = self.get_ask(&token_id);
//
//             // todo: to refactor from here ->
//             let deposit = env::attached_deposit();
//             let panic_msg = format!(
//                 "attached deposit less than token's price.\n\
//                  Attached deposit is {}, token's price is {}",
//                 deposit, sale.price.0
//             );
//             require!(deposit >= sale.price.0, panic_msg);
//
//             require!(
//                 env::prepaid_gas() >= BUY_METHOD_TOTAL_GAS,
//                 format!("attached gas less than: {:?}", BUY_METHOD_TOTAL_GAS)
//             );
//             // <- until here
//             new_owner_id = bidder_id.clone();
//             approval_id = sale.approval_id;
//
//             callback = ext_self::after_nft_transfer_for_ask(
//                 sale,
//                 bidder_id,
//                 env::current_account_id(),
//                 deposit,
//                 AFTER_NFT_TRANSFER_GAS,
//             );
//         }
//         OrderType::AcceptBid {
//             owner_id,
//             approval_id: _approval_id,
//         } => {
//             approval_id = _approval_id;
//
//             let mut ordered_bids = self.bids.get(&token_id).unwrap_or_else(|| {
//                 panic_str(format!("bids for token is: {}, doesn't exists", token_id).as_str())
//             });
//
//             ordered_bids.sort_by_key(|v| v.price.0);
//
//             let last = ordered_bids
//                 .pop()
//                 .unwrap_or_else(|| panic_str("bids is empty"));
//
//             self.bids.insert(&token_id, &ordered_bids);
//
//             new_owner_id = last.bidder_id.clone();
//             callback = ext_self::after_nft_transfer_for_bid(
//                 last,
//                 owner_id,
//                 env::current_account_id(),
//                 env::attached_deposit(),
//                 AFTER_NFT_TRANSFER_GAS,
//             )
//         }
//     };
//
//     ext_nft::nft_transfer(
//         new_owner_id,
//         token_id,
//         Some(approval_id),
//         None,
//         self.nft_id.clone(),
//         ONE_YOCTO,
//         NFT_TRANSFER_GAS,
//     )
//     .then(callback)
// }
//
// fn get_ask(&self, token_id: &TokenId) -> Ask {
//     self.asks.get(token_id).unwrap_or_else(|| {
//         panic_str(format!("token with id {} doesn't sell", token_id).as_str())
//     })
// }
//
// #[private]
// #[payable]
// pub fn after_nft_transfer_for_ask(&mut self, sale: Ask, buyer_id: AccountId) -> Promise {
//     let deposit = env::attached_deposit();
//     match env::promise_result(0) {
//         PromiseResult::Successful(_) => {
//             self.asks.remove(&sale.token_id);
//             let log = json!({
//                 "prev_owner": sale.owner_id,
//                 "curr_owner": buyer_id,
//                 "token_id": sale.token_id,
//                 "price": sale.price,
//             });
//             log!(format!("{}:{}", EVENT_PREFIX, log));
//             self.add_trade_history(sale.clone(), buyer_id.clone());
//
//             let trade = Promise::new(sale.owner_id).transfer(sale.price.0);
//             let change = Promise::new(buyer_id).transfer(deposit - sale.price.0);
//
//             if deposit > sale.price.0 {
//                 log!("bid more than sale price, refund change and paying for sale token");
//                 trade.then(change)
//             } else {
//                 log!("bid equals to sale price, just paying for sale token");
//                 trade
//             }
//         }
//         PromiseResult::Failed => {
//             log!("Execution `nft_transfer` method was failed. Attached deposit was refund.");
//             Promise::new(buyer_id).transfer(deposit)
//         }
//         PromiseResult::NotReady => unreachable!(),
//     }
// }
//
// #[private]
// #[payable]
// pub fn after_nft_transfer_for_bid(
//     &mut self,
//     sale: Bid,
//     owner_id: AccountId,
// ) -> PromiseOrValue<()> {
//     match env::promise_result(0) {
//         PromiseResult::Successful(_) => {
//             let promise = Promise::new(owner_id).transfer(sale.price.0);
//             PromiseOrValue::Promise(promise)
//         }
//         PromiseResult::Failed => {
//             log!("Execution `nft_transfer` method was failed. The bidder's token transfer was stopped.");
//             let mut bids = self.bids.get(&sale.token_id).unwrap_or_else(|| {
//                 panic_str(
//                     format!("bids for token is: {}, doesn't exists", sale.token_id).as_str(),
//                 )
//             });
//             bids.push(sale.clone());
//             self.bids.insert(&sale.token_id, &bids);
//
//             PromiseOrValue::Value(())
//         }
//         PromiseResult::NotReady => unreachable!(),
//     }
// }
//
// #[private]
// #[payable]
// pub fn after_nft_token(
//     &mut self,
//     bidder_id: AccountId,
//     token_id: TokenId,
//     #[rustfmt::skip]
//     #[callback_result]
//     result: Result<Option<TokenExt>, PromiseError>,
// ) -> PromiseOrValue<()> {
//     let bid_price = env::attached_deposit();
//     let ask_less_bid = self
//         .asks
//         .get(&token_id)
//         .map_or(false, |ask| ask.price.0 <= bid_price);
//
//     match result {
//         Ok(Some(_)) if ask_less_bid => {
//             log!("ask for current token id less than provided bid, so process purchase");
//             self.process_purchase(token_id, OrderType::AskLessBid(bidder_id));
//             PromiseOrValue::Value(())
//         }
//         Ok(Some(_)) => {
//             let new_offer_condition = Bid::new(token_id.clone(), bidder_id, bid_price);
//
//             match self.bids.get(&token_id).as_mut() {
//                 Some(offer_conditions) => {
//                     offer_conditions.push(new_offer_condition);
//                     self.bids.insert(&token_id, offer_conditions)
//                 }
//                 _ => self.bids.insert(&token_id, &vec![new_offer_condition]),
//             };
//
//             PromiseOrValue::Value(())
//         }
//         Ok(None) => {
//             log!(
//                 "token with id: {} doesn't exist, attached deposit was returned",
//                 token_id
//             );
//             let promise = Promise::new(bidder_id).transfer(bid_price);
//             PromiseOrValue::Promise(promise)
//         }
//         Err(_) => {
//             log!("`nft_token` execution error was occurred, attached deposit was returned");
//             let promise = Promise::new(bidder_id).transfer(bid_price);
//             PromiseOrValue::Promise(promise)
//         }
//     }
// }

// #[near_bindgen]
// impl NonFungibleTokenApprovalReceiver for Contract {
//     fn nft_on_approve(
//         &mut self,
//         token_id: TokenId,
//         owner_id: AccountId,
//         approval_id: u64,
//         msg: String,
//     ) -> PromiseOrValue<String> {
//         require!(env::predecessor_account_id() == self.nft_id);
//
//         let SaleArgs { sale_type, price } =
//             serde_json::from_str(&msg).expect("Couldn't parse json");
//
//         match sale_type {
//             SaleType::AcceptBid => {
//                 let promise = self.process_purchase(
//                     token_id,
//                     OrderType::AcceptBid {
//                         owner_id,
//                         approval_id,
//                     },
//                 );
//
//                 PromiseOrValue::Promise(promise)
//             }
//             SaleType::Selling => {
//                 let price = price.expect("The price didn't provided for selling").0;
//                 let sale_conditions = Ask::new(owner_id, token_id.clone(), approval_id, price);
//                 self.asks.insert(&token_id, &sale_conditions);
//                 let ret = json!({
//                     "status": true,
//                     "message": format!("token {} with price {} was added to market", token_id, price)
//                 });
//
//                 PromiseOrValue::Value(ret.to_string())
//             }
//         }
//     }
// }
