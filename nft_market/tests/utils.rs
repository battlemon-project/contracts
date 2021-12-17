use near_sdk::serde_json::json;
use near_sdk_sim::lazy_static_include::lazy_static_include_bytes;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{
    call, deploy, init_simulator, ContractAccount, ExecutionResult, UserAccount, STORAGE_AMOUNT,
};

use nft_market::ContractContract as MarketContract;
use nft_token::ContractContract as NFTContract;
use spoiled_nft_token::ContractContract as SpoiledNFTContract;
use test_utils::{
    baz_token_metadata_ext, foo_token_metadata_ext, BASE_DEPOSIT, MARKET_ACCOUNT_ID,
    NFT_ACCOUNT_ID, SPOILED_NFT_ACCOUNT_ID, VALID_TOKEN_ID, VALID_TOKEN_PRICE,
};

type InitAccounts = (
    UserAccount,
    ContractAccount<NFTContract>,
    ContractAccount<MarketContract>,
    UserAccount,
);

type SpoiledInitAccounts = (
    UserAccount,
    ContractAccount<SpoiledNFTContract>,
    ContractAccount<MarketContract>,
    UserAccount,
);

lazy_static_include_bytes! {
    MARKET_WASM => "./../target/wasm32-unknown-unknown/release/nft_market.wasm",
    NFT_WASM => "./../target/wasm32-unknown-unknown/release/nft_token.wasm",
    SPOILED_NFT_WASM => "./../target/wasm32-unknown-unknown/release/spoiled_nft_token.wasm",
}

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

    let alice = root.create_user("alice".parse().unwrap(), *BASE_DEPOSIT);
    (root, nft_contract, market_contract, alice)
}

pub fn init_spoiled() -> SpoiledInitAccounts {
    let root = init_simulator(None);

    let spoiled_nft_contract = deploy!(
        contract: SpoiledNFTContract,
        contract_id: SPOILED_NFT_ACCOUNT_ID,
        bytes: &SPOILED_NFT_WASM,
        signer_account: root,
    );

    let market_contract = deploy!(
        contract: MarketContract,
        contract_id: MARKET_ACCOUNT_ID,
        bytes: &MARKET_WASM,
        signer_account: root,
        init_method: init(SPOILED_NFT_ACCOUNT_ID.parse().unwrap()),
    );

    let alice = root.create_user("alice".parse().unwrap(), *BASE_DEPOSIT);

    (root, spoiled_nft_contract, market_contract, alice)
}

pub trait State {
    fn get_amount(&self) -> u128;
}

impl State for UserAccount {
    fn get_amount(&self) -> u128 {
        self.account()
            .expect("account doesn't contain amount")
            .amount
    }
}

pub trait PromiseResultUtils {
    fn contains_panic_message(&self, message: &str) -> bool;
    fn contains_log(&self, log: &str) -> bool;
}
impl PromiseResultUtils for Vec<Option<ExecutionResult>> {
    fn contains_panic_message(&self, panic_msg: &str) -> bool {
        self.iter()
            .flatten()
            .filter(|v| matches!(v.status(), ExecutionStatus::Failure(e) if e.to_string().contains(panic_msg)))
            .count() == 1
    }

    #[rustfmt::skip]
    fn contains_log(&self, log: &str) -> bool {
        self.iter()
            .flatten()
            .filter(|v| v.logs().contains(&log.to_string()))
            .count() == 1
    }
}

pub fn init_mint_to_alice() -> (
    UserAccount,
    ContractAccount<NFTContract>,
    ContractAccount<MarketContract>,
    UserAccount,
    UserAccount,
) {
    let (root, nft, market, alice) = init();
    let bob = root.create_user("bob".parse().unwrap(), *BASE_DEPOSIT);
    call!(
        nft.user_account,
        nft.mint(
            VALID_TOKEN_ID.to_string(),
            foo_token_metadata_ext(),
            Some(alice.account_id())
        ),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();
    (root, nft, market, alice, bob)
}

pub fn init_mint_to_alice_approve() -> (
    UserAccount,
    ContractAccount<NFTContract>,
    ContractAccount<MarketContract>,
    UserAccount,
    UserAccount,
) {
    let (root, nft, market, alice, bob) = init_mint_to_alice();
    let msg = json!({
        "sale_type": "selling",
        "price": *VALID_TOKEN_PRICE.to_string(),
    })
    .to_string();
    // simulate frontend call for selling nft token.
    call!(
        alice,
        nft.nft_approve(VALID_TOKEN_ID.to_string(), market.account_id(), Some(msg)),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();

    (root, nft, market, alice, bob)
}

pub fn init_mint() -> InitAccounts {
    let (root, nft, market, alice) = init();
    let token_metadata = baz_token_metadata_ext();
    // mint 1 nft token
    call!(
        nft.user_account,
        nft.mint(VALID_TOKEN_ID.to_string(), token_metadata, None),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();
    (root, nft, market, alice)
}

pub fn init_mint_approve() -> InitAccounts {
    let (root, nft, market, alice) = init_mint();
    // try to buy token
    let price = json!({
        "sale_type": "selling",
        "price": *VALID_TOKEN_PRICE.to_string(),
    })
    .to_string();
    // simulate frontend call for selling nft token.
    call!(
        nft.user_account,
        nft.nft_approve(VALID_TOKEN_ID.to_string(), market.account_id(), Some(price)),
        deposit = (STORAGE_AMOUNT / 2)
    )
    .assert_success();

    (root, nft, market, alice)
}
