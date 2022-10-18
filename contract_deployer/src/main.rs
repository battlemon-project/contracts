use anyhow::Context;
use battlemon_models::nft::{ModelKind, NftKind, TokenExt};
use lemotests::prelude::*;
use lemotests::serde_json::json;
use lemotests::Nearable;
use lemotests_macro::add_helpers;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::U128;

pub const NFT_PATH: &str = "./target/wasm32-unknown-unknown/release/nft_token.wasm";
pub const MARKET_PATH: &str = "./target/wasm32-unknown-unknown/release/nft_market.wasm";
pub const NFT: &str = "nft_contract";
pub const MARKET: &str = "market_contract";
pub const NFT_CREDS: &str = "./testnet_creds/nft_contract.json";
pub const MARKET_CREDS: &str = "./testnet_creds/market_contract.json";

add_helpers!("./nft_schema.json", "./market_schema.json",);

async fn nft_mint_testnet() -> anyhow::Result<()> {
    let bchain = StateBuilder::testnet()
        .with_alice(Near(10))?
        .build()
        .await?;

    let alice = bchain.alice_id()?;

    let nft = lemotests::workspaces::Account::from_file("./testnet_creds/nft_contract.json")?;

    bchain
        .alice()?
        .call(bchain.worker(), nft.id(), "nft_mint")
        .args_json(json!({ "receiver_id": alice, "kind": "lemon" }))?
        .max_gas()
        .deposit(Near(1).parse())
        .transact()
        .await?;

    bchain
        .alice()?
        .call(bchain.worker(), nft.id(), "nft_mint")
        .args_json(json!({ "receiver_id": alice, "kind": "fire_arm" }))?
        .max_gas()
        .deposit(Near(1).parse())
        .transact()
        .await?;

    let tokens: Vec<TokenExt> = bchain
        .alice()?
        .call(bchain.worker(), nft.id(), "nft_tokens_for_owner")
        .args_json(json!({ "account_id": alice }))?
        .view()
        .await?
        .json()?;

    let lemon = tokens
        .iter()
        .find(|t| matches!(t.model, ModelKind::Lemon(..)))
        .unwrap();
    let fire_arm = tokens
        .iter()
        .find(|t| matches!(t.model, ModelKind::FireArm(..)))
        .unwrap();

    let instructions = vec![&lemon.token_id, &fire_arm.token_id];
    bchain
        .alice()?
        .call(bchain.worker(), nft.id(), "assemble_compound_nft")
        .args_json(json!({ "instructions": instructions }))?
        .max_gas()
        .deposit(1)
        .transact()
        .await?;

    Ok(())
}

async fn market_sale_testnet() -> anyhow::Result<()> {
    let bchain = StateBuilder::testnet()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, market, alice, _bob] = bchain.string_ids()?;

    println!("nft: {nft}, market: {market}");

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(200))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(200))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_deposit(Near(1))
        .with_gas(Tgas(200))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(200))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(1))
        .with_gas(Tgas(200))
        .then()
        .bob_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(200))
        .with_deposit(required_storage_deposit)
        .then()
        .bob_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(6))
        .with_gas(Tgas(200))
        .execute()
        .await?;

    Ok(())
}

enum Environment {
    TestNet,
    LocalDevelopment,
}

async fn deploy_and_save_creds(env: Environment) -> anyhow::Result<()> {
    let path = match env {
        Environment::TestNet => "./testnet_creds/",
        Environment::LocalDevelopment => "./local_dev_creds/",
    };

    let bchain = StateBuilder::testnet()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(MARKET, MARKET_PATH, Near(10))?
        .with_alice(Near(10))?
        .with_bob(Near(10))?
        .build()
        .await?;

    let [nft, market, alice, _bob] = bchain.string_ids()?;
    println!("nft: {nft}, market: {market}");
    bchain
        .contract(NFT)?
        .as_account()
        .store_credentials(path)
        .await?;

    bchain
        .contract(MARKET)?
        .as_account()
        .store_credentials(path)
        .await?;

    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_market_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Lemon)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Back)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Cap)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::FireArm)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::Cloth)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice, NftKind::ColdArm)?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .view_market_contract_storage_minimum_balance()?
        .with_label("minimum_deposit")
        .execute()
        .await?;

    let required_storage_deposit = result.tx("minimum_deposit")?.json::<U128>()?.0;

    let msg = format!("{{\"price\":\"{}\"}}", Near(5));
    result
        .into_state()
        .alice_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .alice_call_nft_contract_nft_approve("1", &market, Some(&msg))?
        .with_deposit(Near(1))
        .with_gas(Tgas(10))
        .then()
        .bob_call_market_contract_storage_deposit(None)?
        .with_gas(Tgas(10))
        .with_deposit(required_storage_deposit)
        .then()
        .bob_call_market_contract_add_bid("1", None)?
        .with_deposit(Near(6))
        .with_gas(Tgas(200))
        .execute()
        .await?;

    Ok(())
}

async fn try_mint() -> anyhow::Result<()> {
    let bchain = StateBuilder::testnet()
        .with_alice(Near(10))?
        .build()
        .await?;

    let alice = bchain.alice_id()?;

    let nft = lemotests::workspaces::Account::from_file("./testnet_creds/nft_contract.json")?;

    bchain
        .alice()?
        .call(bchain.worker(), nft.id(), "nft_mint")
        .args_json(json!({ "receiver_id": alice, "kind": "lemon" }))?
        .max_gas()
        .deposit(Near(1).parse())
        .transact()
        .await?;

    Ok(())
}

async fn try_mint_full() -> anyhow::Result<()> {
    let bchain = StateBuilder::testnet()
        .with_alice(Near(10))?
        .build()
        .await?;

    let alice = bchain.alice_id()?;

    let nft = lemotests::workspaces::Account::from_file("./testnet_creds/nft_contract.json")?;

    bchain
        .alice()?
        .call(bchain.worker(), nft.id(), "nft_mint_full")
        .args_json(json!({ "receiver_id": alice }))?
        .max_gas()
        .deposit(Near(2).parse())
        .transact()
        .await?;

    Ok(())
}

async fn redeploy(env: Environment) -> anyhow::Result<()> {
    let path_prefix = match env {
        Environment::TestNet => "./testnet_creds/",
        Environment::LocalDevelopment => "./local_dev_creds/",
    };
    let nft_creds = format!("{path_prefix}{NFT}.json");
    let worker = lemotests::workspaces::testnet().await?;
    let nft = lemotests::workspaces::Account::from_file(nft_creds)
        .context("Failed to load creds for nft")?;
    let nft_wasm = tokio::fs::read(NFT_PATH)
        .await
        .context("Failed to read file for nft")?;
    nft.deploy(&worker, &nft_wasm)
        .await
        .context("Failed to deploy nft")?;
    let market_creds = format!("{path_prefix}{MARKET}.json");
    let market = lemotests::workspaces::Account::from_file(market_creds)?;
    let market_wasm = tokio::fs::read(MARKET_PATH).await?;
    market.deploy(&worker, &market_wasm).await?;
    Ok(())
}

async fn try_sale() -> anyhow::Result<()> {
    for _ in 0..10 {
        let bchain = StateBuilder::testnet()
            .with_alice(Near(10))?
            .with_bob(Near(10))?
            .build()
            .await?;

        let alice = bchain.alice_id()?;
        let bob = bchain.bob_id()?;

        let nft = lemotests::workspaces::Account::from_file("./testnet_creds/nft_contract.json")?;
        let market =
            lemotests::workspaces::Account::from_file("./testnet_creds/market_contract.json")?;

        bchain
            .alice()?
            .call(bchain.worker(), nft.id(), "nft_mint")
            .args_json(json!({ "receiver_id": alice, "kind": "lemon" }))?
            .max_gas()
            .deposit(Near(1).parse())
            .transact()
            .await?;

        let tokens: Vec<TokenExt> = bchain
            .alice()?
            .call(bchain.worker(), nft.id(), "nft_tokens_for_owner")
            .args_json(json!({ "account_id": alice }))?
            .view()
            .await?
            .json()?;

        bchain
            .alice()?
            .call(bchain.worker(), market.id(), "storage_deposit")
            .args_json(json!({}))?
            .max_gas()
            .deposit(Near(1).parse())
            .transact()
            .await?;

        let token = tokens.last().unwrap();
        let msg = format!("{{\"price\":\"{}\"}}", Near(5));
        bchain
            .alice()?
            .call(bchain.worker(), nft.id(), "nft_approve")
            .args_json(json!({
                "token_id": token.token_id,
                "account_id": market.id(),
                "msg": msg
            }))?
            .max_gas()
            .deposit(Near(4).parse())
            .transact()
            .await?;

        bchain
            .bob()?
            .call(bchain.worker(), market.id(), "storage_deposit")
            .args_json(json!({}))?
            .max_gas()
            .deposit(Near(1).parse())
            .transact()
            .await?;

        bchain
            .bob()?
            .call(bchain.worker(), market.id(), "add_bid")
            .args_json(json!({
                "token_id": token.token_id,
            }))?
            .max_gas()
            .deposit(Near(7).parse())
            .transact()
            .await?;
    }
    Ok(())
}

async fn add_ten_bids_ten_asks() -> anyhow::Result<()> {
    for _ in 0..10 {
        let bchain = StateBuilder::testnet()
            .with_alice(Near(10))?
            .with_bob(Near(10))?
            .build()
            .await?;

        let alice = bchain.alice_id()?;
        let bob = bchain.bob_id()?;

        let nft = lemotests::workspaces::Account::from_file("./testnet_creds/nft_contract.json")?;
        let market =
            lemotests::workspaces::Account::from_file("./testnet_creds/market_contract.json")?;

        bchain
            .alice()?
            .call(bchain.worker(), nft.id(), "nft_mint")
            .args_json(json!({ "receiver_id": alice, "kind": "lemon" }))?
            .max_gas()
            .deposit(Near(1).parse())
            .transact()
            .await?;

        let tokens: Vec<TokenExt> = bchain
            .alice()?
            .call(bchain.worker(), nft.id(), "nft_tokens_for_owner")
            .args_json(json!({ "account_id": alice }))?
            .view()
            .await?
            .json()?;

        bchain
            .alice()?
            .call(bchain.worker(), market.id(), "storage_deposit")
            .args_json(json!({}))?
            .max_gas()
            .deposit(Near(1).parse())
            .transact()
            .await?;

        let token = tokens.last().unwrap();
        let msg = format!("{{\"price\":\"{}\"}}", Near(5));
        bchain
            .alice()?
            .call(bchain.worker(), nft.id(), "nft_approve")
            .args_json(json!({
                "token_id": token.token_id,
                "account_id": market.id(),
                "msg": msg
            }))?
            .max_gas()
            .deposit(Near(4).parse())
            .transact()
            .await?;

        bchain
            .bob()?
            .call(bchain.worker(), market.id(), "storage_deposit")
            .args_json(json!({}))?
            .max_gas()
            .deposit(Near(1).parse())
            .transact()
            .await?;

        bchain
            .bob()?
            .call(bchain.worker(), market.id(), "add_bid")
            .args_json(json!({
                "token_id": token.token_id,
            }))?
            .max_gas()
            .deposit(Near(3).parse())
            .transact()
            .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // deploy_and_save_creds(Environment::LocalDevelopment).await?;
    // nft_mint_testnet().await?;
    // market_sale_testnet().await?;
    redeploy(Environment::LocalDevelopment).await?;
    // try_sale().await?;
    // add_ten_bids_ten_asks().await?;
    // try_mint_full().await?;

    Ok(())
}
