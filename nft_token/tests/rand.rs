use serde_json::json;
use workspaces::prelude::DevAccountDeployer;

const CONTRACT_WASM: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";

type GenericError = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, GenericError>;

#[tokio::test]
async fn rand() -> Result<()> {
    let worker = workspaces::testnet().await?;
    let wasm = workspaces::compile_project("../nft_token").await?;
    // let wasm = tokio::fs::read(CONTRACT_WASM).await?;
    let account = worker
        .dev_create_account()
        .await
        .expect("could not dev-deploy contract");

    let nft = account
        .create_subaccount(&worker, "nft")
        .initial_balance(50_000_000_000_000_000_000_000_000)
        .transact()
        .await?
        .into_result()?;

    let contract = nft.deploy(&worker, &wasm).await?.into_result()?;
    let res = contract
        .call(&worker, "init")
        .args_json(json!({
                "owner_id": contract.id(),
        }))?
        .transact()
        .await?;
    println!("{:?}", res.outcome());
    let res1 = account
        .call(&worker, contract.id(), "nft_mint")
        .args_json(json!({
            "receiver_id": "f0m0.testnet"
        }))?
        .max_gas()
        .deposit(6470000000000000000000)
        .transact()
        .await?;
    println!("{res1:?}");
    //
    // account
    //     .call(&worker, contract.id(), "nft_mint")
    //     .args_json(json!({
    //         "receiver_id": "battlemon.testnet"
    //     }))?
    //     .gas(300_000_000_000_000)
    //     .deposit(6470000000000000000000)
    //     .transact()
    //     .await?;
    //
    // account
    //     .call(&worker, contract.id(), "nft_mint")
    //     .args_json(json!({
    //         "receiver_id": "battlemon.testnet"
    //     }))?
    //     .gas(300_000_000_000_000)
    //     .deposit(6470000000000000000000)
    //     .transact()
    //     .await?;
    //
    // account
    //     .call(&worker, contract.id(), "nft_mint")
    //     .args_json(json!({
    //         "receiver_id": "battlemon.testnet"
    //     }))?
    //     .gas(300_000_000_000_000)
    //     .deposit(6470000000000000000000)
    //     .transact()
    //     .await?;
    // println!("\nres1: {:#?}", res1);

    Ok(())
}
