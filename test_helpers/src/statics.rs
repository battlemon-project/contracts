use tokio::sync::OnceCell;

static NFT_WASM: OnceCell<Vec<u8>> = OnceCell::const_new();

pub async fn get_nft_wasm(path: &str) -> &'static [u8] {
    NFT_WASM
        .get_or_init(|| async {
            workspaces::compile_project(path)
                .await
                .expect("Failed to compile NFT token contract")
        })
        .await
}