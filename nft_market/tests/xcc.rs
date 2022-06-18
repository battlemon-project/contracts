use near_sdk::serde_json::json;
use test_helpers::workspaces::prelude::*;
use test_helpers::*;

const NFT_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";
const MARKET_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_market.wasm";

#[tokio::test]
async fn alice_ask_for_nft_token_five_bob_bid_six_alice_receive_five_bob_receive_nft_token_and_change_one(
) -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let ten_near = parse_near!("10 N");
    let five_near = parse_near!("5 N");

    let nft = deploy_contract(&worker, "nft", ten_near, &root, NFT_PATH).await?;

    nft.call(&worker, "init")
        .args_json(json!({"owner_id": nft.id()}))?
        .transact()
        .await?;

    let market = deploy_contract(&worker, "market", ten_near, &root, MARKET_PATH).await?;

    market
        .call(&worker, "init")
        .args_json(json!({"nft_id": nft.id()}))?
        .transact()
        .await?;

    let alice = root
        .create_subaccount(&worker, "alice")
        .initial_balance(ten_near)
        .transact()
        .await?
        .into_result()?;

    let bob = root
        .create_subaccount(&worker, "bob")
        .initial_balance(ten_near)
        .transact()
        .await?
        .into_result()?;

    let result = alice
        .call(&worker, nft.id(), "nft_mint")
        .deposit(parse_near!("1 N"))
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let msg = format!("{{\"action\":\"add_ask\",\"price\":\"{five_near}\"}}");
    alice
        .call(&worker, nft.id(), "nft_approve")
        .deposit(parse_near!("1 N"))
        .max_gas()
        .args_json(json!({
            "token_id": "1",
            "account_id": market.id(),
            "msg": msg,
        }))?
        .transact()
        .await?;

    bob.call(&worker, market.id(), "bid")
        .args_json(json!({
            "bid": {
               "token_id": "1"
            }
        }))?
        .max_gas()
        .deposit(parse_near!("6 N"))
        .transact()
        .await?;

    // bob must have nft token
    let nft_token = nft
        .call(&worker, "nft_token")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<token_metadata_ext::TokenExt>()?;
    assert_eq!(nft_token.owner_id.as_str(), bob.id().as_str());

    // bob must have balance ~5N
    let bob_balance = bob.view_account(&worker).await?.balance;
    let diff = parse_near!("5 N") - bob_balance;
    assert!(
        diff <= parse_near!("0.1 N"),
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        bob_balance
    );

    // alice must receive 5N
    let alice_balance = alice.view_account(&worker).await?.balance;
    let diff = parse_near!("15 N") - alice_balance;
    assert!(
        diff <= parse_near!("0.1 N"),
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        alice_balance
    );

    Ok(())
}

#[tokio::test]
async fn bob_bid_six_alice_ask_for_nft_token_five_alice_receive_six_bob_receive_nft_token_and_no_change(
) -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let root = worker.dev_create_account().await?;
    let ten_near = parse_near!("10 N");
    let five_near = parse_near!("5 N");

    let nft = deploy_contract(&worker, "nft", ten_near, &root, NFT_PATH).await?;

    nft.call(&worker, "init")
        .args_json(json!({"owner_id": nft.id()}))?
        .transact()
        .await?;

    let market = deploy_contract(&worker, "market", ten_near, &root, MARKET_PATH).await?;

    market
        .call(&worker, "init")
        .args_json(json!({"nft_id": nft.id()}))?
        .transact()
        .await?;

    let alice = root
        .create_subaccount(&worker, "alice")
        .initial_balance(ten_near)
        .transact()
        .await?
        .into_result()?;

    let bob = root
        .create_subaccount(&worker, "bob")
        .initial_balance(ten_near)
        .transact()
        .await?
        .into_result()?;

    let result = alice
        .call(&worker, nft.id(), "nft_mint")
        .deposit(parse_near!("1 N"))
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    bob.call(&worker, market.id(), "bid")
        .args_json(json!({
            "bid": {
               "token_id": "1"
            }
        }))?
        .max_gas()
        .deposit(parse_near!("6 N"))
        .transact()
        .await?;

    let msg = format!("{{\"action\":\"add_ask\",\"price\":\"{five_near}\"}}");
    alice
        .call(&worker, nft.id(), "nft_approve")
        .deposit(parse_near!("1 N"))
        .max_gas()
        .args_json(json!({
            "token_id": "1",
            "account_id": market.id(),
            "msg": msg,
        }))?
        .transact()
        .await?;

    // bob must have nft token
    let nft_token = nft
        .call(&worker, "nft_token")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<token_metadata_ext::TokenExt>()?;
    assert_eq!(nft_token.owner_id.as_str(), bob.id().as_str());

    // bob must have balance <=4N
    let bob_balance = bob.view_account(&worker).await?.balance;
    let diff = parse_near!("4 N") - bob_balance;
    assert!(
        diff <= parse_near!("0.1 N"),
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        bob_balance
    );

    // alice must receive 6N
    let alice_balance = alice.view_account(&worker).await?.balance;
    let diff = parse_near!("16 N") - alice_balance;
    assert!(
        diff <= parse_near!("0.1 N"),
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        alice_balance
    );

    Ok(())
}
