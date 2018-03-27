pub mod api;
pub mod schema;
pub mod service;
pub mod transactions;
pub mod wallet;

pub use self::api::*;
pub use self::schema::*;
pub use self::service::*;
pub use self::transactions::*;
pub use self::wallet::*;


const SERVICE_ID: u16 = 2;
const SERVICE_NAME: &str = "copper";
const TX_CREATE_WALLET_ID: u16 = 1;
const TX_TRANSFER_ID: u16 = 2;
const INIT_BALANCE: u64 = 100;


