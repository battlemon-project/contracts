#!/bin/bash

set -e
if [ -d "./neardev" ]; then
  rm -rf ./neardev;
fi
echo n | near dev-deploy ./target/wasm32-unknown-unknown/release/play2earn.wasm

source ./neardev/dev-account.env
export JUICE=juice.$CONTRACT_NAME
export PLAYER=player.$CONTRACT_NAME
export PROGRESS_PROVIDER=progress_provider.$CONTRACT_NAME

near create-account $JUICE --masterAccount $CONTRACT_NAME --initialBalance 10
near create-account $PLAYER --masterAccount $CONTRACT_NAME --initialBalance 10
near create-account $PROGRESS_PROVIDER --masterAccount $CONTRACT_NAME --initialBalance 10

near call $CONTRACT_NAME init '{"juice_id": "'$JUICE'", "progress_provider_id": "'$PROGRESS_PROVIDER'"}' --accountId $CONTRACT_NAME

near deploy $JUICE ./target/wasm32-unknown-unknown/release/juice.wasm --masterAccount $CONTRACT_NAME
near call $JUICE init '{"owner_id": "'$CONTRACT_NAME'", "total_supply": "1000000"}' --accountId $CONTRACT_NAME

near call $CONTRACT_NAME process_progress '{"progress": {"player_id": "'$PLAYER'", "played_time": 10, "total_damage": 20, "hp_level": 30, "walking_distance": 40, "match_result": true}}' --accountId $PROGRESS_PROVIDER --deposit 1 --gas 300000000000000