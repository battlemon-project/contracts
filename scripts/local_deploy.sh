#!/bin/bash

set -e
if [ -d "./neardev" ]; then
  rm -rf ./neardev
fi

export KEY_PATH=/near-configs/validator_key.json

echo n | near dev-deploy ./target/wasm32-unknown-unknown/release/nft_token.wasm --keyPath=$KEY_PATH

source ./neardev/dev-account.env
export MARKET=market.$CONTRACT_NAME

near call $CONTRACT_NAME init '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME --keyPath=$KEY_PATH

near call $CONTRACT_NAME mint '{"token_id": "1", "token_metadata": {"title": "Title for token 1", "description": "some description for battlemon nft token", "media": "http://some-link-to-media.com", "model": {"lemon": {"option": "on_sale", "century": "our_time", "type": "light", "lemon_gen": "nakamoto", "background": "red", "top": "headdress", "cyber_suit": "metallic", "expression": "brooding", "eyes": "open", "hair": "bob_marley", "accessory": "cigar", "winrate": 14, "rarity": 12, "slots": []}}}}' --accountId $CONTRACT_NAME --amount 0.1 --keyPath=$KEY_PATH
#near call $CONTRACT_NAME  mint '{"token_id": "2", "token_metadata": {"title": "Title for token 2", "description": "some description for battlemon nft token", "media": "http://some-link-to-media.com", "model": {"weapon": {"level": 1, "type": "projection", "slots": []}}}}' --accountId $CONTRACT_NAME --amount 0.1
#near call $CONTRACT_NAME  mint '{"token_id": "3", "token_metadata": {"title": "Title for token 3", "description": "some description for battlemon nft token", "media": "http://some-link-to-media.com", "model": {"suppressor": {"slots": []}}}}' --accountId $CONTRACT_NAME --amount 0.1

near create-account $MARKET --masterAccount $CONTRACT_NAME --initialBalance 50 --keyPath=$KEY_PATH
near deploy $MARKET ./target/wasm32-unknown-unknown/release/nft_market.wasm --masterAccount $CONTRACT_NAME --keyPath=$KEY_PATH
near call $MARKET init '{"nft_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME --keyPath=$KEY_PATH
#
near call $CONTRACT_NAME nft_approve '{"token_id": "1", "account_id": "'$MARKET'", "msg": "{\"sale_type\":\"selling\",\"price\":\"10\"}"}' --accountId $CONTRACT_NAME --depositYocto 510000000000000000000 --keyPath=$KEY_PATH

export NEW_OWNER_ID=alice.$CONTRACT_NAME
near create-account $NEW_OWNER_ID --masterAccount $CONTRACT_NAME --initialBalance 30 --keyPath=$KEY_PATH
near call $MARKET buy '{"token_id": "1"}' --depositYocto 10 --gas 90000000000000 --accountId $NEW_OWNER_ID --keyPath=$KEY_PATH
#near call $MARKET buy '{"token_id": "1"}' --depositYocto 10 --gas 200000000000000 --accountId alice.dev-1636529128471-59911444209733
#near call $MARKET bid '{"token_id": "1"}' --accountId $CONTRACT_NAME --depositYocto 510000000000000000000 --gas 200000000000000
