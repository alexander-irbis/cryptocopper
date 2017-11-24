extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate exonum;
extern crate exonum_configuration;
extern crate router;
extern crate bodyparser;
extern crate iron;

pub mod service;

use exonum::helpers::fabric::NodeBuilder;
use exonum_configuration::ConfigurationServiceFactory;

use service::currency;



fn main() {
    exonum::helpers::init_logger().unwrap();
    NodeBuilder::new()
        .with_service(Box::new(ConfigurationServiceFactory))
        .with_service(Box::new(currency::CurrencyServiceFactory))
        .run();
}
