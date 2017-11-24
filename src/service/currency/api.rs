use bodyparser;
use exonum::blockchain::{Blockchain, Transaction};
use exonum::api::{Api, ApiError};
use exonum::crypto::{PublicKey, Hash, HexValue};
use exonum::node::{TransactionSend, ApiSender};
use serde_json;

use iron::prelude::*;
use router::Router;


use super::tx::*;
use super::wallet::*;
use super::schema::*;


#[serde(untagged)]
#[derive(Clone, Serialize, Deserialize)]
pub enum TransactionRequest {
    CreateWallet(TxCreateWallet),
    Transfer(TxTransfer),
}

impl Into<Box<Transaction>> for TransactionRequest {
    fn into(self) -> Box<Transaction> {
        match self {
            TransactionRequest::CreateWallet(trans) => Box::new(trans),
            TransactionRequest::Transfer(trans) => Box::new(trans),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    tx_hash: Hash,
}


#[derive(Clone)]
pub struct CryptocurrencyApi {
    pub channel: ApiSender,
    pub blockchain: Blockchain,
}


impl Api for CryptocurrencyApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let tx_handler = move |req: &mut Request| -> IronResult<Response> {
            match req.get::<bodyparser::Struct<TransactionRequest>>() {
                Ok(Some(tx)) => {
                    let tx: Box<Transaction> = tx.into();
                    let tx_hash = tx.hash();
                    self_.channel.send(tx).map_err(ApiError::from)?;
                    let json = TransactionResponse { tx_hash };
                    self_.ok_response(&serde_json::to_value(&json).unwrap())
                }
                Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
            }
        };

        // Gets status of all wallets in the database.
        let self_ = self.clone();
        let wallets_info = move |_: &mut Request| -> IronResult<Response> {
            if let Some(wallets) = self_.get_wallets() {
                self_.ok_response(&serde_json::to_value(wallets).unwrap())
            } else {
                self_.not_found_response(
                    &serde_json::to_value("Wallets database is empty")
                        .unwrap(),
                )
            }
        };

        // Gets status of the wallet corresponding to the public key.
        let self_ = self.clone();
        let wallet_info = move |req: &mut Request| -> IronResult<Response> {
            let path = req.url.path();
            let wallet_key = path.last().unwrap();
            let public_key = PublicKey::from_hex(wallet_key).map_err(ApiError::FromHex)?;
            if let Some(wallet) = self_.get_wallet(&public_key) {
                self_.ok_response(&serde_json::to_value(wallet).unwrap())
            } else {
                self_.not_found_response(
                    &serde_json::to_value("Wallet not found").unwrap(),
                )
            }
        };

        router.post("/v1/wallets/transaction", tx_handler, "transaction");
        router.get("/v1/wallets", wallets_info, "wallets_info");
        router.get("/v1/wallet/:pub_key", wallet_info, "wallet_info");
    }
}


impl CryptocurrencyApi {
    fn get_wallet(&self, pub_key: &PublicKey) -> Option<Wallet> {
        let mut view = self.blockchain.fork();
        let mut schema = CurrencySchema::new(&mut view);
        schema.wallet(pub_key)
    }

    fn get_wallets(&self) -> Option<Vec<Wallet>> {
        let mut view = self.blockchain.fork();
        let mut schema = CurrencySchema::new(&mut view);
        let idx = schema.wallets();
        let wallets: Vec<Wallet> = idx.values().collect();
        if wallets.is_empty() {
            None
        } else {
            Some(wallets)
        }
    }
}
