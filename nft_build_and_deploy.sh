#!/bin/bash

set -e

CONTRACT="battlemon.testnet"
NFT_CONTRACT="nft.${CONTRACT}"

cargo build --target wasm32-unknown-unknown --release

near delete $NFT_CONTRACT $CONTRACT
near create-account $NFT_CONTRACT --masterAccount $CONTRACT
near deploy $NFT_CONTRACT ./target/wasm32-unknown-unknown/release/nft_token.wasm --masterAccount $CONTRACT --initDeposit 1
near call $NFT_CONTRACT init '{"owner_id": "'$NFT_CONTRACT'"}' --accountId $CONTRACT

IMAGE_URL="https://battlemon.com/fighters.png"

for i in {1..10}
do
  near call $NFT_CONTRACT mint '{"token_id": "'$i'", "token_metadata": {"title": "Title for token '$i'", "description": "some description for batllemon nft token", "media": "'$IMAGE_URL'"}}' --accountId $NFT_CONTRACT --amount 0.1
done