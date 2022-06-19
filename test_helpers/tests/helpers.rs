use near_units::parse_near;
use test_helpers::workspaces::testnet;
use test_helpers::StateBuilder;

#[tokio::test]
async fn builder_works() -> Result<(), anyhow::Error> {
    StateBuilder::new(testnet())
        .with_alice(parse_near!("10 N"))?
        .with_bob(parse_near!("10 N"))?
        .with_contract(
            "nft",
            "../target/wasm32-unknown-unknown/release/nft_token.wasm",
            parse_near!("10 N"),
        )?
        .build()
        .await?;

    Ok(())
}
