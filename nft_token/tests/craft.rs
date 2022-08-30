use lemotests::{Near, StateBuilder, Tgas};
use lemotests_macro::add_helpers;
use near_sdk::json_types::U128;
use token_metadata_ext::TokenExt;

const NFT_PATH: &str = "../target/wasm32-unknown-unknown/release/nft_token.wasm";
const NFT: &str = "nft_contract";
const JUICE_PATH: &str = "../target/wasm32-unknown-unknown/release/juice.wasm";
const JUICE: &str = "juice_contract";
add_helpers!("./nft_schema.json", "./juice_schema.json");

#[tokio::test]
async fn mint_works() -> anyhow::Result<()> {
    let bchain = StateBuilder::testnet()
        .with_contract(NFT, NFT_PATH, Near(10))?
        .with_contract(JUICE, JUICE_PATH, Near(10))?
        .with_alice(Near(10))?
        .build()
        .await?;

    let [nft, juice, alice] = bchain.string_ids()?;
    let msg = serde_json::json!({
       "tokens_ids": ["1"],
    });
    let result = bchain
        .call_nft_contract_init(&nft)?
        .with_gas(Tgas(10))
        .then()
        .call_juice_contract_init(&juice, U128(100000))?
        .with_gas(Tgas(10))
        .then()
        .alice_call_nft_contract_nft_mint(&alice)?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .call_juice_contract_storage_deposit(Some(&alice))?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .call_juice_contract_storage_deposit(Some(&nft))?
        .with_gas(Tgas(10))
        .with_deposit(Near(1))
        .then()
        .call_juice_contract_ft_transfer(&alice, U128(10000), None)?
        .with_gas(Tgas(10))
        .with_deposit(1)
        .then()
        .alice_call_juice_contract_ft_transfer_call(&nft, U128(1000), &msg.to_string())?
        .with_deposit(1)
        .with_gas(Tgas(50))
        .then()
        .view_nft_contract_nft_token("2")?
        .with_label("view_nft_token")
        .execute()
        .await?;

    // let nft_token = result.tx("view_nft_token")?.json::<TokenExt>()?;
    dbg!(juice);

    Ok(())
}
