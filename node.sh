#!/bin/sh

export RUST_LOG="exonum::node=debug"
./target/debug/cryptocopper run --node-config srv/etc/validators/$1.toml --db-path srv/dat/db/$1 --public-api-address 127.0.0.1:800$1 --private-api-address 127.0.0.1:810$1
