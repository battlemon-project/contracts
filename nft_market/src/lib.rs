pub use ask::*;
use battlemon_models::helpers_contract::emit_log_event;
use battlemon_models::market::events::MarketEventKind;
use battlemon_models::market::{ask::AskForContract, bid::BidForContract};
pub use bid::*;
use consts::*;
use error::*;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::store::{LookupMap, UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise};
use std::ops::AddAssign;

mod ask;
mod bid;
mod consts;
mod error;
mod external;
mod helpers;
mod trade;
mod xcc;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    nft_id: AccountId,
    asks: UnorderedMap<TokenId, AskForContract>,
    bids: UnorderedMap<TokenId, Vec<BidForContract>>,
    storage_deposits: LookupMap<AccountId, Balance>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Asks,
    Bids,
    StorageDeposits,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(nft_id: AccountId) -> Self {
        Self {
            nft_id,
            asks: UnorderedMap::new(StorageKey::Asks),
            bids: UnorderedMap::new(StorageKey::Bids),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
        }
    }

    #[payable]
    #[handle_result]
    pub fn add_bid(
        &mut self,
        token_id: TokenId,
        expire_at: Option<u64>,
    ) -> Result<(), ContractError> {
        self.check_storage_deposits(&env::predecessor_account_id())?;

        let bid = BidForContract::new(token_id, expire_at);
        match self.ask_less_than_bid(&bid) {
            None => {
                emit_log_event(MarketEventKind::AddBid(bid.to_owned()));

                self.bids
                    .entry(bid.token_id().to_owned())
                    .and_modify(|bids| {
                        bids.push(bid.clone());
                    })
                    .or_insert_with(|| vec![bid]);
            }
            Some(ask) => self.trade(ask, bid, true),
        }

        Ok(())
    }

    pub fn ask(&self, token_id: TokenId) -> Option<&AskForContract> {
        self.asks.get(&token_id)
    }

    #[handle_result]
    pub fn cancel_ask(&mut self, token_id: TokenId) -> Result<(), ContractError> {
        let ask = self
            .asks
            .get(&token_id)
            .ok_or(ContractError::NotFound("Ask doesn't exist"))?
            .to_owned();

        if ask.account_id() != &env::predecessor_account_id() {
            return Err(ContractError::NotAuthorized(
                "Ask's owner is not the same as the caller",
            ));
        }

        self.asks.remove(&token_id);
        emit_log_event(MarketEventKind::RemoveAsk(ask));

        Ok(())
    }

    pub fn bids(&self, token_id: TokenId) -> Option<&Vec<BidForContract>> {
        self.bids.get(&token_id)
    }

    #[handle_result]
    pub fn cancel_bid(&mut self, token_id: TokenId, bid_id: String) -> Result<(), ContractError> {
        let bids = self.bids.get_mut(&token_id).ok_or(ContractError::NotFound(
            "Bids for provided token id don't exist",
        ))?;

        let idx = bids
            .iter()
            .position(|b| b.id == bid_id)
            .ok_or(ContractError::NotFound(
                "Bid with provided id doesn't exist",
            ))?;
        let bid = bids.get(idx).unwrap();
        if bid.account_id() != &env::predecessor_account_id() {
            return Err(ContractError::NotAuthorized(
                "Ask's owner is not the same as the caller",
            ));
        }

        let bid = bids.swap_remove(idx);
        emit_log_event(MarketEventKind::RemoveBid(bid));

        bids.is_empty().then(|| self.bids.remove(&token_id));

        Ok(())
    }

    #[payable]
    #[handle_result]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) -> Result<(), ContractError> {
        let storage_account_id = account_id.unwrap_or_else(env::predecessor_account_id);
        let deposit = env::attached_deposit();
        check_deposit(deposit)?;

        self.storage_deposits
            .entry(storage_account_id)
            .or_default()
            .add_assign(deposit);

        Ok(())
    }

    #[payable]
    #[handle_result]
    pub fn storage_withdraw(&mut self) -> Result<U128, ContractError> {
        check_one_yocto()?;
        let owner_id = env::predecessor_account_id();
        let deposit = self.storage_deposits.remove(&owner_id).unwrap_or_default();

        let amount_orders = self.total_orders_by_id(&owner_id);
        let effective_deposit = (amount_orders as u128) * STORAGE_PER_SALE;
        let diff = deposit - effective_deposit;

        if diff > 0 {
            Promise::new(owner_id.clone()).transfer(diff);
        }

        if effective_deposit > 0 {
            self.storage_deposits.insert(owner_id, effective_deposit);
        }

        Ok(U128(diff))
    }

    pub fn storage_minimum_balance(&self) -> U128 {
        U128(STORAGE_PER_SALE)
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        let ret = self
            .storage_deposits
            .get(&account_id)
            .copied()
            .unwrap_or_default();

        U128(ret)
    }
}

fn check_one_yocto() -> Result<(), ContractError> {
    if env::attached_deposit() != 1 {
        return Err(ContractError::OneYoctoDeposit);
    }

    Ok(())
}

fn check_deposit(deposit: Balance) -> Result<(), ContractError> {
    if deposit < STORAGE_PER_SALE {
        return Err(ContractError::InsufficientDeposit);
    }

    Ok(())
}

#[cfg(all(not(target_arch = "wasm32"), test))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    const DEPOSIT: Balance = 1960000000000000000000;

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

    #[test]
    fn storage_deposit_works() {
        let mut context = get_context(accounts(1));
        context.attached_deposit(DEPOSIT);
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        contract.storage_deposit(None).unwrap();

        assert_eq!(contract.storage_deposits.get(&accounts(1)), Some(&DEPOSIT));
    }

    #[test]
    fn storage_deposit_sums_correctly() {
        let mut context = get_context(accounts(1));
        context.attached_deposit(DEPOSIT);
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        contract.storage_deposit(None).unwrap();
        contract.storage_deposit(None).unwrap();

        assert_eq!(
            contract.storage_deposits.get(&accounts(1)),
            Some(&(DEPOSIT * 2))
        );
    }

    #[test]
    fn storage_deposit_fails_if_insufficient_deposit() {
        let mut context = get_context(accounts(1));
        context.attached_deposit(DEPOSIT);
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(0));
        contract.storage_deposit(None).unwrap();
        context.attached_deposit(DEPOSIT - 1);
        testing_env!(context.build());

        matches!(
            contract.storage_deposit(None),
            Err(ContractError::InsufficientDeposit)
        );
    }
}
// pub fn list_asks(&self) -> Vec<AskForContract> {
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
// pub fn list_bids(&self) -> Vec<(TokenId, Vec<BidForContract>)> {
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
// fn get_ask(&self, token_id: &TokenId) -> AskForContract {
//     self.asks.get(token_id).unwrap_or_else(|| {
//         panic_str(format!("token with id {} doesn't sell", token_id).as_str())
//     })
// }
//
// #[private]
// #[payable]
// pub fn after_nft_transfer_for_ask(&mut self, sale: AskForContract, buyer_id: AccountId) -> Promise {
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
//     sale: BidForContract,
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
//             let new_offer_condition = BidForContract::new(token_id.clone(), bidder_id, bid_price);
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
//                 let sale_conditions = AskForContract::new(owner_id, token_id.clone(), approval_id, price);
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
