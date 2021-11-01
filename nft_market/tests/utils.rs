use near_sdk_sim::lazy_static_include::lazy_static_include_bytes;

lazy_static_include_bytes! {
    NFT_MARKET_WASM => "./../target/wasm32-unknown-unknown/release/nft_market.wasm",
}