use near_sdk::{AccountId, BlockHeight};
use serde_json::{json, Value};
use workspaces::prelude::DevAccountDeployer;
use workspaces::Function;

const CONTRACT_WASM: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";

type GenericError = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, GenericError>;

#[tokio::test]
async fn rand() -> Result<()> {
    let worker = workspaces::testnet();
    let wasm = tokio::fs::read(CONTRACT_WASM).await?;
    let account = worker
        .dev_create_account()
        .await
        .expect("could not dev-deploy contract");

    let nft = account
        .create_subaccount(&worker, "nft")
        .initial_balance(100_000_000_000_000_000_000_000_000)
        .transact()
        .await?
        .into_result()?;

    let contract = nft.deploy(&worker, &wasm).await?.into_result()?;

    contract
        .call(&worker, "init")
        .args_json(json!({
                "owner_id": contract.id(),
        }))?
        .transact()
        .await?;

    contract
        .call(&worker, "nft_mint")
        .args_json(json!({
            "receiver_id": "battlemon.testnet"
        }))?
        .deposit(6470000000000000000000)
        .transact()
        .await?;

    contract
        .call(&worker, "nft_mint")
        .args_json(json!({
            "receiver_id": "battlemon.testnet"
        }))?
        .deposit(6470000000000000000000)
        .transact()
        .await?;

    let result1 = contract
        .view(
            &worker,
            "nft_tokens",
            json!({"from_index": null, "limit": null})
                .to_string()
                .into_bytes(),
        )
        .await?
        .json::<Value>()?;

    println!("{}", result1);

    Ok(())
}
