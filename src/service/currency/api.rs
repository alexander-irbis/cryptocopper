use bodyparser;
use exonum::blockchain::{Blockchain, Transaction};
use exonum::api::{Api, ApiError};
use exonum::crypto::{PublicKey, Hash};
use exonum::encoding::serialize::FromHex;
use exonum::node::{TransactionSend, ApiSender};
use serde::Deserialize;
use serde_json;

use iron::headers::ContentType;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status::Status;
use router::Router;

use super::transactions::*;
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
        let post_create_wallet =
            move |req: &mut Request| self_.post_transaction::<TxCreateWallet>(req);
        let self_ = self.clone();
        let post_transfer = move |req: &mut Request| self_.post_transaction::<TxTransfer>(req);
        let self_ = self.clone();
        let get_wallets = move |req: &mut Request| self_.get_wallets(req);
        let self_ = self.clone();
        let get_wallet = move |req: &mut Request| self_.get_wallet(req);

        // Bind handlers to specific routes.
        router.post("/v1/wallets", post_create_wallet, "post_create_wallet");
        router.post("/v1/wallets/transfer", post_transfer, "post_transfer");
        router.get("/v1/wallets", get_wallets, "get_wallets");
        router.get("/v1/wallet/:pub_key", get_wallet, "get_wallet");
    }
}


impl CryptocurrencyApi {
    /// Endpoint for getting a single wallet.
    fn get_wallet(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let wallet_key = path.last().unwrap();
        let public_key = PublicKey::from_hex(wallet_key).map_err(|e| {
            IronError::new(e, (
                Status::BadRequest,
                Header(ContentType::json()),
                r#""Invalid request param: `pub_key`""#,
            ))
        })?;

        let wallet = {
            let mut snapshot = self.blockchain.snapshot();
            let mut schema = CurrencySchema::new(&snapshot);
            schema.wallet(&public_key)
        };

        if let Some(wallet) = wallet {
            self.ok_response(&serde_json::to_value(wallet).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("Wallet not found").unwrap())
        }
    }

    /// Endpoint for dumping all wallets from the storage.
    fn get_wallets(&self, _: &mut Request) -> IronResult<Response> {
        let mut view = self.blockchain.fork();
        let schema = CurrencySchema::new(&mut view);
        let idx = schema.wallets();
        let wallets: Vec<Wallet> = idx.values().collect();

        self.ok_response(&serde_json::to_value(&wallets).unwrap())
    }

    /// Common processing for transaction-accepting endpoints.
    fn post_transaction<T>(&self, req: &mut Request) -> IronResult<Response>
        where
            T: Transaction + Clone + for<'de> Deserialize<'de>,
    {
        match req.get::<bodyparser::Struct<T>>() {
            Ok(Some(transaction)) => {
                let transaction: Box<Transaction> = Box::new(transaction);
                let tx_hash = transaction.hash();
                self.channel.send(transaction).map_err(ApiError::from)?;
                let json = TransactionResponse { tx_hash };
                self.ok_response(&serde_json::to_value(&json).unwrap())
            }
            Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        }
    }
}
