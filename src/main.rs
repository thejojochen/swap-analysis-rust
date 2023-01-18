//tutorial-read-serde-04.rs
use csv::{ReaderBuilder, StringRecord};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::num;
use std::process;
mod tmstmpcnv;
use std::collections::HashMap;
// This lets us write `#[derive(Deserialize)]`.
use serde::Deserialize;

// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    iD: String,
    transaction_ID: String,
    timestamp: String, //in pst**
    timestamp_Unix: String,
    pool: String,
    sender: String,
    recipient: String,
    origin: String,
    Token0Symbol: String,
    Token1Symbol: String,
    Token0: String,
    Token1: String,
    SqrticeX96: Option<String>,
    Tick: String,
    Log_Index: String,
    Amount0: f64,
    Amount1: f64,
    AmountUSD: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct priceData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: String,
    mktcap: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_nth_arg(1)?;
    let file = File::open(file_path)?;
    println!("analyzing {:?}", file);
    
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let file_path2 = get_nth_arg(2)?;
    let file2 = File::open(file_path2)?;

    let mut rdr2 = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file2);

    let large_trade_threshold: f64 = get_nth_arg(3)?.into_string().ok().unwrap().parse()?; //could handle errors better?
    let mut bigGains: u64 = 0;
    let mut smallLosses: u64 = 0;
    let mut smallGains: u64 = 0;
    let mut bigLosses: u64 = 0;
    let mut breakeven: u64 = 0;
    let mut dateToClosePrice = HashMap::new();

    // variables for second analysis
    let mut agg_gains_large: f64 = 0.0;
    let mut agg_gains_small: f64 = 0.0;
    let mut agg_losses_large: f64 = 0.0;
    let mut agg_losses_small: f64 = 0.0;



    for result2 in rdr2.deserialize() {
        let record2: priceData = result2?;
        //println!("time: {}, close price: {}", record2.date, record2.close);
        dateToClosePrice.insert(record2.date, record2.close);
    }

    for result in rdr.deserialize() {
        let record: Record = result?;
        let &effPrice = &(record.Amount0 / record.Amount1).abs();


        //println!("{}", record.timestamp);
        let mut convertedTimestamp: String = tmstmpcnv::convert_timestamp(record.timestamp);
        //println!("new converted timestamp: {}", convertedTimestamp);
        let mut closePrice: f64 = dateToClosePrice[&convertedTimestamp];
        let mut priceDiff: f64 = &closePrice - &effPrice;

        let tradeVol: &f64 = &record.AmountUSD;
        let mut tradeType: u64 = 0;
        if &record.Amount1 > &0.0 {
            tradeType = 1 //buy side
        } else {
            tradeType = 2; // sell side 
        }
        if tradeType == 1 {
            if priceDiff > 0.0 {
                if tradeVol > &large_trade_threshold {
                    bigGains += 1;
                } else {
                    smallGains += 1;
                }
            } else {
                if tradeVol > &large_trade_threshold {
                    bigLosses += 1;
                } else {
                    smallLosses += 1;
                }
            }
        } else if tradeType == 2 {
            if priceDiff < 0.0 {
                if tradeVol > &large_trade_threshold {
                    bigGains += 1;
                } else {
                    smallGains += 1;
                }
            } else {
                if tradeVol > &large_trade_threshold {
                    bigLosses += 1;
                } else {
                    smallLosses += 1;
                }
            }
        } else {
            breakeven += 1;
        }

        // ANALYSIS TWO (aggregate gain and losses) 
        //need to figure out math here
        // percentage change
        let percentage_change = ((&closePrice - &effPrice) / &effPrice);
        //println!("{}", percentage_change);
        let usd_diff: f64 = (tradeVol * percentage_change).abs();
        //println!("{}", usd_diff);
        
        if percentage_change > 0.0 {
            // price went up
            if (tradeType == 1) {
                //gain
                if (tradeVol > &large_trade_threshold) {agg_gains_large += usd_diff;} else {agg_gains_small += usd_diff;}

        } else if tradeType == 2 {
                //loss
                if (tradeVol > &large_trade_threshold) {agg_losses_large += usd_diff;} else {agg_losses_small += usd_diff;}
            }
        } else if percentage_change < 0.0 {
            //price went down
            if tradeType == 1 {
                //loss
                if (tradeVol > &large_trade_threshold) {agg_losses_large += usd_diff;} else {agg_losses_small += usd_diff;}
            } else if tradeType == 2 {
                if (tradeVol > &large_trade_threshold) {agg_gains_large += usd_diff;} else {agg_gains_small += usd_diff;}
        }
        } else {
            return Err(From::from("problem with aggregation"));
        }
    }
    println!(
        "bigGains:{}, smallGains:{}, bigLosses:{}, smallLosses{}, breakeven:{}",
        bigGains, smallGains, bigLosses, smallLosses, breakeven
    );

    println!("aggregate usd total for large trades: {}", agg_gains_large - agg_losses_large);
    println!("aggregate usd total for small trades: {}", agg_gains_small - agg_losses_small);
    println!("currently does not account for time zone conversion, needs unix -> date conversion");
    println!("");

    Ok(())
}

/// Returns the nth positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_nth_arg(index: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(index) {
        None => Err(From::from("no arguments supplied")),
        Some(file_path_or_threshold) => Ok(file_path_or_threshold),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// Try this if you don't like each record smushed on one line:
// println!("{:#?}", record);
