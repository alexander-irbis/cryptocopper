#!/bin/sh

mkdir -p srv/etc
mkdir -p srv/dat
./target/debug/cryptocopper generate-testnet --start 6000 $1 --output_dir srv/etc/

