use exonum::crypto::PublicKey;
use exonum::storage::{Fork, MapIndex};

use super::wallet::Wallet;


pub struct CurrencySchema<'a> {
    view: &'a mut Fork,
}

impl<'a> CurrencySchema<'a> {
    pub fn new(view: &'a mut Fork) -> Self {
        CurrencySchema { view }
    }

    pub fn wallets(&mut self) -> MapIndex<&mut Fork, PublicKey, Wallet> {
        MapIndex::new("copper.wallets", self.view)
    }

    // Utility method to quickly get a separate wallet from the storage
    pub fn wallet(&mut self, pub_key: &PublicKey) -> Option<Wallet> {
        self.wallets().get(pub_key)
    }
}
