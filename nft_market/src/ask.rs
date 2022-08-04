use battlemon_models::market::ask_contract::Ask;
use battlemon_models::market::bid_contract::Bid;
use near_sdk::AccountId;

impl crate::Contract {
    /// Add ask for a concrete token.
    ///
    /// The market automatically completes the trade
    /// if the asker provides a price less than the highest bid.
    /// First, the bidder receives the asker's token.
    /// Then, the asker gets the bidder's Nears held by the market.
    pub(crate) fn add_ask(&mut self, ask: &Ask) {
        match self.highest_bid_than_ask(ask) {
            None => {
                self.asks.insert(ask.token_id().to_owned(), ask.to_owned());
                // todo: emit ask log here
            }
            Some(bid) => self.trade(ask.to_owned(), bid, false),
        }
    }

    pub(crate) fn ask_less_than_bid(&self, bid: &Bid) -> Option<Ask> {
        self.asks
            .get(bid.token_id())
            .filter(|ask| ask.price() <= bid.price())
            .cloned()
    }

    pub(crate) fn count_asks_for_account(&self, account_id: &AccountId) -> usize {
        self.asks
            .iter()
            .filter(|(_, ask)| ask.account_id() == account_id)
            .count()
    }
}
