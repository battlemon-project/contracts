#!/bin/bash

set -e

near dev-deploy ./target/wasm32-unknown-unknown/release/nft_token.wasm -f

source ./neardev/dev-account.env

near call $CONTRACT_NAME init '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME

for i in {1..10}
do
  near call $CONTRACT_NAME  mint '{"token_id": "'$i'", "token_metadata": {"title": "Title for token '$i'", "description": "some description for battlemon nft token", "media": "http://some-link-to-media.com", "properties": {"option": "on_sale", "century": "our_time", "type": "light", "lemon_gen": "nakamoto", "background": "red", "top": "headdress", "cyber_suit": "metallic", "expression": "brooding", "eyes": "open", "hair": "bob_marley", "accessory": "cigar", "winrate": 14, "rarity": 12}}}' --accountId $CONTRACT_NAME --amount 0.1
done