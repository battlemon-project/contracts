// use battlemon_near_json_rpc_client_wrapper::{JsonRpcWrapper, NEAR_TESTNET_ARCHIVAL_RPC_URL};
// use tokio::sync::OnceCell;

// static TESTNET_JSON_RPC_WRAPPER: OnceCell<JsonRpcWrapper> = OnceCell::const_new();

// pub async fn get_testnet_json_rpc_wrapper() -> &'static JsonRpcWrapper {
//     TESTNET_JSON_RPC_WRAPPER
//         .get_or_init(|| async { JsonRpcWrapper::connect(NEAR_TESTNET_ARCHIVAL_RPC_URL) })
//         .await
// }
