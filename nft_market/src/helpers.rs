use near_sdk::AccountId;

impl crate::Contract {
    pub(crate) fn total_orders_by_id(&self, id: &AccountId) -> usize {
        self.count_bids_for_account(id) + self.count_asks_for_account(id)
    }
}
