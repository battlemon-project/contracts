use crate::external::*;
use crate::{Ask, Bid};
use near_sdk::env;

impl crate::Contract {
    /// * asker - wants near for token
    /// * bidder - gives near for token
    ///
    /// # Examples:
    /// ## Asker offers less than the highest bid:
    /// *Given:* Bidder has offered 2 Near for the token,
    /// asker haven't seen that and call `add_ask` for
    /// the token with the price equals 1.
    ///
    /// *Result:* Asker must receive the bidder's two Near,
    /// and the bidder receives the asker's token.
    ///
    /// ## Bidder offers more than token's price:
    /// *Given:* Asker has requested 1 Near for the token,
    /// the bidder hasn't seen that and calls `add_bid`
    /// for the token with the price equaling 2 Near.
    ///
    /// *Result:* Asker must receive a bidder's 1 Near,
    /// and the bidder accepts the asker's token and 1 Near for change.
    pub(crate) fn trade(&mut self, ask: Ask, bid: Bid, change: bool) {
        nft::ext(self.nft_id.clone())
            .with_attached_deposit(env::attached_deposit())
            .with_static_gas(env::prepaid_gas())
            .nft_transfer(
                bid.account_id().to_owned(),
                bid.token_id().to_owned(),
                ask.approval_id(),
                None,
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_attached_deposit(env::attached_deposit())
                    .with_static_gas(env::prepaid_gas())
                    .on_trade(ask, bid, change),
            );
    }
}

// self.trade(Ask(1), Bid(2));
// self.trade(Ask(1), Bid(2));
