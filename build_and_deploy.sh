set -e

cargo build --target wasm32-unknown-unknown --release

near delete nft.battlemon.testnet battlemon.testnet \
  && near create-account nft.battlemon.testnet --masterAccount battlemon.testnet \
  || near create-account nft.battlemon.testnet --masterAccount battlemon.testnet

near deploy nft.battlemon.testnet ./target/wasm32-unknown-unknown/release/nft_token.wasm --masterAccount battlemon.testnet --initDeposit 1

near call nft.battlemon.testnet init '{"owner_id": "nft.battlemon.testnet"}'

near call nft.battlemon.testnet mint '{"token_id": "1", "token_metadata": {"title": "title for 1", "description": "some description for batllemon nft token", "media": "some url"}}' --accountId nft.battlemon.testnet --amount 0.1
