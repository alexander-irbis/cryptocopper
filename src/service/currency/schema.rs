use exonum::crypto::PublicKey;
use exonum::storage::{Fork, MapIndex, Snapshot};

use super::wallet::Wallet;

pub struct CurrencySchema<T> {
    view: T,
}

impl<T: AsRef<Snapshot>> CurrencySchema<T> {
    pub fn new(view: T) -> Self {
        CurrencySchema { view }
    }

    pub fn wallets(&self) -> MapIndex<&Snapshot, PublicKey, Wallet> {
        MapIndex::new("copper.wallets", self.view.as_ref())
    }

    // Utility method to quickly get a separate wallet from the storage
    pub fn wallet(&self, pub_key: &PublicKey) -> Option<Wallet> {
        self.wallets().get(pub_key)
    }
}

impl<'a> CurrencySchema<&'a mut Fork> {
    /// Returns a mutable version of the wallets table.
    pub fn wallets_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Wallet> {
        MapIndex::new("copper.wallets", self.view)
    }
}