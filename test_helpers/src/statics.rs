use std::path::Path;
// use tokio::sync::OnceCell;
//
// static WASM: OnceCell<Vec<u8>> = OnceCell::const_new();
//
// pub async fn load_wasm(path: impl AsRef<Path>) -> &'static [u8] {
//     WASM.get_or_init(|| async {
//         tokio::fs::read(path)
//             .await
//             .expect("Failed to load wasm file.")
//     })
//     .await
// }
