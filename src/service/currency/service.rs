use exonum::api::Api;
use exonum::blockchain::{Service, Transaction, ApiContext};
use exonum::crypto::Hash;
use exonum::encoding;
use exonum::helpers::fabric::{ServiceFactory, Context};
use exonum::messages::RawTransaction;
use exonum::storage::Snapshot;

use iron::Handler;
use router::Router;


use super::*;


/// A currency service creator for the `NodeBuilder`
#[derive(Debug)]
pub struct CurrencyServiceFactory;

#[derive(Debug)]
pub struct CurrencyService;


impl ServiceFactory for CurrencyServiceFactory {
    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(CurrencyService)
    }
}


impl Service for CurrencyService {
    fn service_name(&self) -> &'static str {
        SERVICE_NAME
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let trans: Box<Transaction> = match raw.message_type() {
            TX_TRANSFER_ID => Box::new(TxTransfer::from_raw(raw)?),
            TX_CREATE_WALLET_ID => Box::new(TxCreateWallet::from_raw(raw)?),
            _ => {
                return Err(encoding::Error::IncorrectMessageType {
                    message_type: raw.message_type(),
                });
            }
        };
        Ok(trans)
    }

    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = CryptocurrencyApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
        };
        api.wire(&mut router);
        Some(Box::new(router))
    }

    fn state_hash(&self, _snapshot: &Snapshot) -> Vec<Hash> {
        Vec::new()
    }
}
