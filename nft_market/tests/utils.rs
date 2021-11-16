use near_sdk_sim::lazy_static_include::lazy_static_include_bytes;
use near_sdk_sim::{deploy, init_simulator, to_yocto, ContractAccount, UserAccount};
use nft_market::ContractContract as MarketContract;
use nft_token::ContractContract as NFTContract;

lazy_static_include_bytes! {
    MARKET_WASM => "./../target/wasm32-unknown-unknown/release/nft_market.wasm",
    NFT_WASM => "./../target/wasm32-unknown-unknown/release/nft_token.wasm",
}

pub const ONE_YOCTO: u128 = 1;
const MARKET_ACCOUNT_ID: &str = "market";
const NFT_ACCOUNT_ID: &str = "nft";

pub fn init() -> (
    UserAccount,
    ContractAccount<NFTContract>,
    ContractAccount<MarketContract>,
    UserAccount,
) {
    let root = init_simulator(None);

    let nft_contract = deploy!(
        contract: NFTContract,
        contract_id: NFT_ACCOUNT_ID,
        bytes: &NFT_WASM,
        signer_account: root,
        init_method: init(NFT_ACCOUNT_ID.parse().unwrap()),
    );

    let market_contract = deploy!(
        contract: MarketContract,
        contract_id: MARKET_ACCOUNT_ID,
        bytes: &MARKET_WASM,
        signer_account: root,
        init_method: init(NFT_ACCOUNT_ID.parse().unwrap()),
    );

    let alice = root.create_user("alice".parse().unwrap(), to_yocto("100"));
    (root, nft_contract, market_contract, alice)
}
