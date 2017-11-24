#!/bin/sh

./target/debug/copper run --node-config srv/etc/validators/$1.toml --rocksdb srv/dat/db/$1 --public-api-address 127.0.0.1:800$1 --private-api-address 127.0.0.1:810$1
