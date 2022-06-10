// #[tokio::test]
// async fn mint_sell_buy() -> Result<()> {
//     let worker = workspaces::testnet().await?;
//     let nft_market_wasm = workspaces::compile_project("../nft_market").await?;
//     let root_acc = worker
//         .dev_create_account()
//         .await
//         .expect("could not dev-deploy contract");
//
//     let nft_token_acc = root_acc
//         .create_subaccount(&worker, "nft")
//         .initial_balance(parse_near!("10 N"))
//         .transact()
//         .await?
//         .into_result()?;
//     let nft_token_contract = nft_token_acc
//         .deploy(&worker, &nft_token_wasm)
//         .await?
//         .into_result()?;
//     let init_res = nft_token_contract
//         .call(&worker, "init")
//         .args_json(json!({
//                 "owner_id": nft_token_acc.id(),
//         }))?
//         .transact()
//         .await?;
//     let crypto_hash = CryptoHash(init_res.outcome().block_hash.0);
//     let block_height = get_testnet_json_rpc_wrapper()
//         .await
//         .block_height_from_hash(crypto_hash)
//         .await?;
//
//     let main_account_id = root_acc.id();
//     println!("Block height: {block_height}\nMain account id: {main_account_id}");
//     let market_acc = root_acc
//         .create_subaccount(&worker, "market")
//         .initial_balance(parse_near!("10 N"))
//         .transact()
//         .await?
//         .into_result()?;
//     let market_acc_contract = market_acc
//         .deploy(&worker, &nft_market_wasm)
//         .await?
//         .into_result()?;
//     market_acc_contract
//         .call(&worker, "init")
//         .args_json(json!({
//             "nft_id": nft_token_contract.id()
//         }))?
//         .transact()
//         .await?;
//     let alice_acc = root_acc
//         .create_subaccount(&worker, "alice")
//         .initial_balance(parse_near!("10 N"))
//         .transact()
//         .await?
//         .into_result()?;
//
//     let mint_res = alice_acc
//         .call(&worker, nft_token_contract.id(), "nft_mint")
//         .args_json(json!({
//             "receiver_id": alice_acc.id()
//         }))?
//         .max_gas()
//         .deposit(6470000000000000000000)
//         .transact()
//         .await?;
//     let minted_token = mint_res.json::<TokenExt>()?;
//     let token_price = parse_near!("1 N");
//     let selling_message = json!({
//         "sale_type": "selling",
//         "price": token_price.to_string(),
//     })
//     .to_string();
//
//     alice_acc
//         .call(&worker, nft_token_contract.id(), "nft_approve")
//         .args_json(json!({
//             "token_id": minted_token.token_id,
//             "account_id": market_acc_contract.id(),
//             "msg": selling_message,
//         }))?
//         .max_gas()
//         .deposit(520000000000000000000)
//         .transact()
//         .await?;
//
//     let bob_acc = root_acc
//         .create_subaccount(&worker, "bob")
//         .initial_balance(parse_near!("10 N"))
//         .transact()
//         .await?
//         .into_result()?;
//     bob_acc
//         .call(&worker, market_acc_contract.id(), "buy")
//         .args_json(json!({
//             "token_id": minted_token.token_id,
//         }))?
//         .max_gas()
//         .deposit(token_price)
//         .transact()
//         .await?;
//
//     Ok(())
// }
