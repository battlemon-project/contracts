// use near_sdk::{AccountId, Gas};
// use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS};
// use nft_market::Ask;
// use test_utils::{INVALID_TOKEN_ID, INVALID_TOKEN_PRICE, VALID_TOKEN_ID, VALID_TOKEN_PRICE};
// use utils::{PromiseResultUtils, State};
// 
// mod utils;
// 
// #[test]
// fn buying_token_with_invalid_id_must_panic() {
//     let (_root, _nft, market, alice) = utils::init_mint_approve();
//     let exec_result = call!(
//         alice,
//         market.buy(INVALID_TOKEN_ID.to_string()),
//         *VALID_TOKEN_PRICE,
//         DEFAULT_GAS
//     );
// 
//     assert_eq!(exec_result.promise_errors().len(), 1);
//     assert!(exec_result
//         .promise_results()
//         .contains_panic_message(&format!("token with id {} doesn't sell", INVALID_TOKEN_ID)));
// }
// 
// #[test]
// fn buying_token_with_price_less_than_required_must_panic() {
//     let (_root, _nft, market, alice) = utils::init_mint_approve();
//     let exec_result = call!(
//         alice,
//         market.buy(VALID_TOKEN_ID.to_string()),
//         *INVALID_TOKEN_PRICE,
//         DEFAULT_GAS
//     );
//     let panic_msg = format!(
//         "attached deposit less than token's price.\nAttached deposit is {}, token's price is {}",
//         *INVALID_TOKEN_PRICE, *VALID_TOKEN_PRICE
//     );
//     assert_eq!(exec_result.promise_errors().len(), 1);
//     assert!(exec_result
//         .promise_results()
//         .contains_panic_message(&panic_msg));
// }
// 
// #[test]
// fn buying_token_with_attached_deposit_equals_to_token_price() {
//     let (_root, nft, market, alice, bob) = utils::init_mint_to_alice_approve();
//     let initial_balance = alice.get_amount();
//     // simulate buying process from user
//     let exec_results = call!(
//         bob,
//         market.buy(VALID_TOKEN_ID.to_string()),
//         *VALID_TOKEN_PRICE,
//         DEFAULT_GAS
//     );
//     let actual_balance = alice.get_amount();
// 
//     exec_results.assert_success();
//     assert_eq!(exec_results.promise_errors().len(), 0);
//     // checking that asks is empty
//     let sale_conditions: Vec<Ask> = view!(market.list_asks()).unwrap_json();
//     assert_eq!(sale_conditions.len(), 0);
//     // checking that new owner is Bob
//     let owner_id: AccountId =
//         view!(nft.get_owner_by_token_id(VALID_TOKEN_ID.to_string())).unwrap_json();
//     assert_eq!(owner_id, bob.account_id());
//     // checking that seller got near.
//     let diff = actual_balance - initial_balance;
//     assert_eq!(diff, *VALID_TOKEN_PRICE)
// }
// 
// #[test]
// fn buying_token_with_attached_deposit_more_than_token_price_must_refund_diff() {
//     let (_root, nft, market, _alice, bob) = utils::init_mint_to_alice_approve();
//     let initial_balance = bob.get_amount();
//     // simulate buying process from user
//     let exec_results = call!(
//         bob,
//         market.buy(VALID_TOKEN_ID.to_string()),
//         *VALID_TOKEN_PRICE * 2,
//         DEFAULT_GAS
//     );
//     let actual_balance = bob.get_amount();
// 
//     exec_results.assert_success();
//     assert_eq!(exec_results.promise_errors().len(), 0);
//     // checking that asks is empty
//     let sale_conditions: Vec<Ask> = view!(market.list_asks()).unwrap_json();
//     assert_eq!(sale_conditions.len(), 0);
//     // checking that new owner is Bob
//     let owner_id: AccountId =
//         view!(nft.get_owner_by_token_id(VALID_TOKEN_ID.to_string())).unwrap_json();
//     assert_eq!(owner_id, bob.account_id());
//     // checking that seller got near.
//     let diff = initial_balance - actual_balance;
//     assert!((diff - *VALID_TOKEN_PRICE) < to_yocto("0.1"));
// }
