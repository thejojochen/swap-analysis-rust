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
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut rdr2 = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    let mut bigGains: u64 = 0;
    let mut smallLosses: u64 = 0;
    let mut smallGains: u64 = 0;
    let mut bigLosses: u64 = 0;
    let mut breakeven: u64 = 0;
    let mut dateToClosePrice = HashMap::new();

    println!("starting deserializaiton");

    for result2 in rdr2.deserialize() {
        let record2: priceData = result2?;
        //println!("time: {}, close price: {}", record2.date, record2.close);
        dateToClosePrice.insert(record2.date, record2.close);
    }

    println!("starting analysis");

    for result in rdr.deserialize() {
        let record: Record = result?;
        let &effPrice = &(record.Amount0 / record.Amount1).abs();
        //let closePrice: &f64 = lookUpPrice(record.timestamp); // for now

        //println!("{}", record.timestamp);
        let mut convertedTimestamp: String = tmstmpcnv::convert_timestamp(record.timestamp);
        let mut closePrice: f64 = dateToClosePrice[&convertedTimestamp];
        let mut priceDiff: f64 = &closePrice - &effPrice;

        //println!("starting to print yays");
        //println!("{}", dateToClosePrice[&convertedTimestamp]);
        //if(!dateToClosePrice.contains_key(&convertedTimestamp)) {println!("yay");} else {println!("nay");}

        //let priceDiff: f64 = *&closePrice - effPrice;
        let tradeVol: &f64 = &record.AmountUSD;
        let mut tradeType: u64 = 0;
        if &record.Amount1 > &0.0 {
            tradeType = 1
        } else {
            tradeType = 2;
        }
        if tradeType == 1 {
            if priceDiff > 0.0 {
                if tradeVol > &5000.0 {
                    bigGains += 1;
                } else {
                    smallGains += 1;
                }
            } else {
                if tradeVol > &5000.0 {
                    bigLosses += 1;
                } else {
                    smallLosses += 1;
                }
            }
        } else if tradeType == 2 {
            if priceDiff < 0.0 {
                if tradeVol > &5000.0 {
                    bigGains += 1;
                } else {
                    smallGains += 1;
                }
            } else {
                if tradeVol > &5000.0 {
                    bigLosses += 1;
                } else {
                    smallLosses += 1;
                }
            }
        } else {
            breakeven += 1;
        }
    }
    println!(
        "bigGains:{}, smallGains:{}, bigLosses:{}, smallLosses{}, breakeven:{}",
        bigGains, smallGains, bigLosses, smallLosses, breakeven
    );
    Ok(())
}

/// Returns the nth positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
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
