use exonum::crypto::PublicKey;


encoding_struct! {
    struct Wallet {
        pub_key: &PublicKey,
        name: &str,
        balance: u64,
    }
}

impl Wallet {
    pub fn increase(self, amount: u64) -> Self {
        let balance = self.balance() + amount;
        Self::new(self.pub_key(), self.name(), balance)
    }

    pub fn decrease(self, amount: u64) -> Self {
        let balance = self.balance() - amount;
        Self::new(self.pub_key(), self.name(), balance)
    }
}

