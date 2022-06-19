use crate::{HelperError, NFT};
use crate::{State, MARKET};
use near_sdk::serde_json::json;
use workspaces::Network;

impl<T> State<T>
where
    T: Network,
{
    pub async fn init_contracts(&self) -> Result<(), HelperError> {
        let nft = self.contract(NFT)?;

        self.contract(MARKET)?
            .call(self.worker(), "init")
            .args_json(json!({"nft_id": nft.id()}))?
            .transact()
            .await?;

        self.contract(NFT)?
            .call(self.worker(), "init")
            .args_json(json!({"owner_id": nft.id()}))?
            .transact()
            .await?;

        Ok(())
    }
}
