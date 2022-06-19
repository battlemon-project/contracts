use near_sdk::serde_json::json;
use nft_market::{Ask, Bid};
use test_helpers::*;

#[tokio::test]
async fn alice_ask_for_nft_token_five_bob_bid_six_alice_receive_five_bob_receive_nft_token_and_change_one(
) -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let state = StateBuilder::new(worker)
        .with_alice(TEN_NEAR)?
        .with_bob(TEN_NEAR)?
        .with_contract(NFT, NFT_PATH, TEN_NEAR)?
        .with_contract(MARKET, MARKET_PATH, TEN_NEAR)?
        .build()
        .await?;
    state.init_contracts().await?;
    let (alice, bob, worker) = (state.alice()?, state.bob()?, state.worker());
    let (nft, market) = (state.contract(NFT)?, state.contract(MARKET)?);

    let result = alice
        .call(worker, nft.id(), "nft_mint")
        .deposit(ONE_NEAR)
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let msg = format!("{{\"action\":\"add_ask\",\"price\":\"{FIVE_NEAR}\"}}");
    alice
        .call(worker, nft.id(), "nft_approve")
        .deposit(ONE_NEAR)
        .max_gas()
        .args_json(json!({
            "token_id": "1",
            "account_id": market.id(),
            "msg": msg,
        }))?
        .transact()
        .await?;

    bob.call(worker, market.id(), "add_bid")
        .args_json(json!({
            "bid": {
               "token_id": "1"
            }
        }))?
        .max_gas()
        .deposit(SIX_NEAR)
        .transact()
        .await?;

    // bob must have nft token
    let nft_token = nft
        .call(worker, "nft_token")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<token_metadata_ext::TokenExt>()?;
    assert_eq!(nft_token.owner_id.as_str(), bob.id().as_str());

    // bob must have balance ~5N
    let bob_balance = bob.view_account(worker).await?.balance;
    let diff = FIVE_NEAR - bob_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        bob_balance
    );

    // alice must receive 5N
    let alice_balance = alice.view_account(worker).await?.balance;
    let diff = FIFTEEN_NEAR - alice_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        alice_balance
    );

    Ok(())
}

#[tokio::test]
async fn bob_bid_six_alice_ask_for_nft_token_five_alice_receive_six_bob_receive_nft_token_and_no_change(
) -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let state = StateBuilder::new(worker)
        .with_alice(TEN_NEAR)?
        .with_bob(TEN_NEAR)?
        .with_contract(NFT, NFT_PATH, TEN_NEAR)?
        .with_contract(MARKET, MARKET_PATH, TEN_NEAR)?
        .build()
        .await?;
    state.init_contracts().await?;
    let (alice, bob, worker) = (state.alice()?, state.bob()?, state.worker());
    let (nft, market) = (state.contract(NFT)?, state.contract(MARKET)?);

    let result = alice
        .call(worker, nft.id(), "nft_mint")
        .deposit(ONE_NEAR)
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    bob.call(worker, market.id(), "add_bid")
        .args_json(json!({"bid": {"token_id": "1"}}))?
        .max_gas()
        .deposit(SIX_NEAR)
        .transact()
        .await?;

    let msg = format!("{{\"action\":\"add_ask\",\"price\":\"{FIVE_NEAR}\"}}");
    alice
        .call(worker, nft.id(), "nft_approve")
        .deposit(ONE_NEAR)
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
        .call(worker, "nft_token")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<token_metadata_ext::TokenExt>()?;
    assert_eq!(nft_token.owner_id.as_str(), bob.id().as_str());

    // bob must have balance <=4N
    let bob_balance = bob.view_account(worker).await?.balance;
    let diff = FOUR_NEAR - bob_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        bob_balance
    );

    // alice must receive 6N
    let alice_balance = alice.view_account(worker).await?.balance;
    let diff = SIXTEEN_NEAR - alice_balance;
    assert!(
        diff <= ALMOST_ZERO,
        "Expected bob balance isn't less than 0.1 N, actual balance is {}",
        alice_balance
    );

    Ok(())
}

#[tokio::test]
async fn bid_first_ask_second_then_trade_then_bid_is_removed_and_ask_does_not_created(
) -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let state = StateBuilder::new(worker)
        .with_alice(TEN_NEAR)?
        .with_bob(TEN_NEAR)?
        .with_contract(NFT, NFT_PATH, TEN_NEAR)?
        .with_contract(MARKET, MARKET_PATH, TEN_NEAR)?
        .build()
        .await?;
    state.init_contracts().await?;
    let (alice, bob, worker) = (state.alice()?, state.bob()?, state.worker());
    let (nft, market) = (state.contract(NFT)?, state.contract(MARKET)?);

    let result = alice
        .call(worker, nft.id(), "nft_mint")
        .deposit(ONE_NEAR)
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    bob.call(worker, market.id(), "add_bid")
        .args_json(json!({"bid": {"token_id": "1"}}))?
        .max_gas()
        .deposit(SIX_NEAR)
        .transact()
        .await?;

    let bids = market
        .call(worker, "bids")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<Option<Vec<Bid>>>()?;

    assert_eq!(bids.unwrap().len(), 1);

    let msg = format!("{{\"action\":\"add_ask\",\"price\":\"{FIVE_NEAR}\"}}");
    alice
        .call(worker, nft.id(), "nft_approve")
        .deposit(ONE_NEAR)
        .max_gas()
        .args_json(json!({
            "token_id": "1",
            "account_id": market.id(),
            "msg": msg,
        }))?
        .transact()
        .await?;

    let ask = market
        .call(worker, "ask")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<Option<Ask>>()?;

    assert_eq!(ask, None);

    let bids = market
        .call(worker, "bids")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<Option<Vec<Bid>>>()?;

    assert_eq!(bids.unwrap().len(), 0);

    Ok(())
}

#[tokio::test]
async fn ask_first_bid_second_then_trade_then_ask_is_removed_and_bid_does_not_created(
) -> anyhow::Result<()> {
    let worker = workspaces::testnet().await?;
    let state = StateBuilder::new(worker)
        .with_alice(TEN_NEAR)?
        .with_bob(TEN_NEAR)?
        .with_contract(NFT, NFT_PATH, TEN_NEAR)?
        .with_contract(MARKET, MARKET_PATH, TEN_NEAR)?
        .build()
        .await?;
    state.init_contracts().await?;
    let (alice, bob, worker) = (state.alice()?, state.bob()?, state.worker());
    let (nft, market) = (state.contract(NFT)?, state.contract(MARKET)?);

    let result = alice
        .call(worker, nft.id(), "nft_mint")
        .deposit(ONE_NEAR)
        .args_json(json!({"receiver_id": alice.id()}))?
        .transact()
        .await?;
    assert!(result.is_success());

    let msg = format!("{{\"action\":\"add_ask\",\"price\":\"{FIVE_NEAR}\"}}");
    alice
        .call(worker, nft.id(), "nft_approve")
        .deposit(ONE_NEAR)
        .max_gas()
        .args_json(json!({
            "token_id": "1",
            "account_id": market.id(),
            "msg": msg,
        }))?
        .transact()
        .await?;

    let ask = market
        .call(worker, "ask")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<Option<Ask>>()?;

    assert_eq!(ask.unwrap().token_id(), "1");

    bob.call(worker, market.id(), "add_bid")
        .args_json(json!({"bid": {"token_id": "1"}}))?
        .max_gas()
        .deposit(SIX_NEAR)
        .transact()
        .await?;

    let bids = market
        .call(worker, "bids")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<Option<Vec<Bid>>>()?;

    assert_eq!(bids, None);

    let ask = market
        .call(worker, "ask")
        .args_json(json!({"token_id": "1"}))?
        .view()
        .await?
        .json::<Option<Ask>>()?;

    assert_eq!(ask, None);

    Ok(())
}