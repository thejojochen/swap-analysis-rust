#!/bin/sh

# Author : Zara Ali
# Copyright (c) Tutorialspoint.com
# Script follows here:


# cargo run data/ETH-USDC-WETH-005-62.csv ethpriceshistorical.csv
# cargo run data/ETH-USDC-WETH-005-62.csv ethpriceshistorical.csv

# ./target/debug/swap-analysis-rust data/ETH-USDC-WETH-005-62.csv ethpriceshistorical.csv

for file in data/*; do
    ./target/debug/swap-analysis-rust data/$(basename "$file") ethpriceshistorical.csv
done