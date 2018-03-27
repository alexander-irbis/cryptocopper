extern crate bodyparser;
#[macro_use]
extern crate exonum;
extern crate exonum_configuration;
extern crate iron;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod service;

use exonum::helpers::fabric::NodeBuilder;

use service::currency;


fn main() {
    exonum::helpers::init_logger().unwrap();
    NodeBuilder::new()
        .with_service(Box::new(exonum_configuration::ServiceFactory))
        .with_service(Box::new(currency::ServiceFactory))
        .run();
}
