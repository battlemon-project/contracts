#!/bin/bash

set -e


MAIN_CONTRACT="battlemon.testnet"
MARKET_CONTRACT="market.${MAIN_CONTRACT}"
NFT_CONTRACT="nft.${MAIN_CONTRACT}"

cargo build -p nft_market --target wasm32-unknown-unknown --release

near delete $MARKET_CONTRACT $MAIN_CONTRACT
near create-account $MARKET_CONTRACT --masterAccount $MAIN_CONTRACT
near deploy $MARKET_CONTRACT ./target/wasm32-unknown-unknown/release/nft_market.wasm --masterAccount $MAIN_CONTRACT --initDeposit 50
near call $MARKET_CONTRACT init '{"nft_id": "'$NFT_CONTRACT'"}' --accountId $MAIN_CONTRACT