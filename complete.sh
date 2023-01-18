#!/bin/sh

# Author : Jonas C

# cargo run data/ETH-USDC-WETH-005-62.csv ethpriceshistorical.csv
# cargo run data/ETH-USDC-WETH-005-62.csv ethpriceshistorical.csv
# ./target/debug/swap-analysis-rust data/ETH-USDC-WETH-005-62.csv ethpriceshistorical.csv

cargo build 
for file in data/*; do
    ./target/debug/swap-analysis-rust data/$(basename "$file") ethpriceshistorical.csv 5000.0
done

# argument structure
# 1: target file to analyze
# 2: csv of historical eth prices
# 3: large trade thershold of type f64