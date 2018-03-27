use exonum::crypto::PublicKey;
use exonum::blockchain::{Transaction, ExecutionResult};
use exonum::storage::Fork;
use exonum::messages::Message;


use super::*;


transactions! {
    CurrencyTransactions {
        const SERVICE_ID = SERVICE_ID;

        struct TxCreateWallet {
            pub_key: &PublicKey,
            name: &str,
        }

        struct TxTransfer {
            from: &PublicKey,
            to: &PublicKey,
            amount: u64,
            seed: u64,
        }
    }
}


impl Transaction for TxCreateWallet {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(view);
        if schema.wallet(self.pub_key()).is_none() {
            let wallet = Wallet::new(self.pub_key(), self.name(), INIT_BALANCE);
            println!("Create the wallet: {:?}", wallet);
            schema.wallets_mut().put(self.pub_key(), wallet)
        }
        Ok(())
    }
}


impl Transaction for TxTransfer {
    fn verify(&self) -> bool {
        (*self.from() != *self.to()) &&
            self.verify_signature(self.from())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(view);
        let sender = schema.wallet(self.from());
        let receiver = schema.wallet(self.to());
        if let (Some(sender), Some(receiver)) = (sender, receiver) {
            let amount = self.amount();
            if sender.balance() >= amount {
                let sender = sender.decrease(amount);
                let receiver = receiver.increase(amount);
                println!("Transfer between wallets: {:?} => {:?}",
                    sender,
                    receiver);
                let mut wallets = schema.wallets_mut();
                wallets.put(self.from(), sender);
                wallets.put(self.to(), receiver);
            }
        }
        Ok(())
    }
}
