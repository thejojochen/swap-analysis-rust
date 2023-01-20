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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    id: String,
    transaction_id: String,
    timestamp: String, //in pst**
    timestamp_unix: i64,
    pool: String,
    sender: String,
    recipient: String,
    origin: String,
    token0_symbol: String,
    token1_symbol: String,
    token0: String,
    token1: String,
    sqrt_price_x96: Option<String>,
    tick: String,
    log_index: String,
    amount0: f64,
    amount1: f64,
    amount_usd: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PriceData {
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
    let mut big_gains: u64 = 0;
    let mut small_losses: u64 = 0;
    let mut small_gains: u64 = 0;
    let mut big_losses: u64 = 0;
    let mut breakeven: u64 = 0;
    let mut date_to_close_price = HashMap::new();

    // variables for second analysis
    let mut agg_gains_large: f64 = 0.0;
    let mut agg_gains_small: f64 = 0.0;
    let mut agg_losses_large: f64 = 0.0;
    let mut agg_losses_small: f64 = 0.0;



    for result2 in rdr2.deserialize() {
        let record2: PriceData = result2?;
        //println!("time: {}, close price: {}", record2.date, record2.close);
        date_to_close_price.insert(record2.date, record2.close);
    }

    for result in rdr.deserialize() {
        let record: Record = result?;
        let &eff_price = &(record.amount0 / record.amount1).abs();


        //println!("{}", record.timestamp);
        let converted_timestamp: String = tmstmpcnv::convert_timestamp(record.timestamp_unix);
        //println!("new converted timestamp: {}", converted_timestamp);
        let close_price: f64 = date_to_close_price[&converted_timestamp];
        let price_diff: f64 = &close_price - &eff_price;

        let trade_vol: &f64 = &record.amount_usd;
        let mut trade_type: u64 = 0;
        if &record.amount1 > &0.0 {
            trade_type = 1 //buy side (enum po)
        } else {
            trade_type = 2; // sell side 
        }
        if trade_type == 1 {
            if price_diff > 0.0 {
                if trade_vol > &large_trade_threshold {
                    big_gains += 1;
                } else {
                    small_gains += 1;
                }
            } else {
                if trade_vol > &large_trade_threshold {
                    big_losses += 1;
                } else {
                    small_losses += 1;
                }
            }
        } else if trade_type == 2 {
            if price_diff < 0.0 {
                if trade_vol > &large_trade_threshold {
                    big_gains += 1;
                } else {
                    small_gains += 1;
                }
            } else {
                if trade_vol > &large_trade_threshold {
                    big_losses += 1;
                } else {
                    small_losses += 1;
                }
            }
        } else {
            breakeven += 1;
        }

        // ANALYSIS TWO (aggregate gain and losses) 
        // percentage change
        let percentage_change = (&close_price - &eff_price) / &eff_price;
        //println!("{}", percentage_change);
        let usd_diff: f64 = (trade_vol * percentage_change).abs();
        //println!("{}", usd_diff);
        if usd_diff == std::f64::INFINITY {continue;} //next: check if usd_diff is a valid f64 (using tmstmpvnv::type_of)
        
        if percentage_change > 0.0 {
            // price went up
            if trade_type == 1 {
                //gain
                if trade_vol > &large_trade_threshold {agg_gains_large += usd_diff;} else {agg_gains_small += usd_diff;}

        } else if trade_type == 2 {
                //loss
                if trade_vol > &large_trade_threshold {agg_losses_large += usd_diff;} else {agg_losses_small += usd_diff;}
            }
        } else if percentage_change < 0.0 {
            //price went down
            if trade_type == 1 {
                //loss
                if trade_vol > &large_trade_threshold {agg_losses_large += usd_diff;} else {agg_losses_small += usd_diff;}
            } else if trade_type == 2 {
                if trade_vol > &large_trade_threshold {agg_gains_large += usd_diff;} else {agg_gains_small += usd_diff;}
        }
        } else {
            return Err(From::from("problem with aggregation")); 
        }
    }
    println!(
        "big_gains:{}, small_gains:{}, big_losses:{}, small_losses{}, breakeven:{}",
        big_gains, small_gains, big_losses, small_losses, breakeven
    );

    if (agg_gains_small - agg_losses_small == std::f64::INFINITY) {println!("inf value");}
    println!("aggregate usd total for large trades: {}", agg_gains_large - agg_losses_large);
    println!("aggregate usd total for small trades: {}", agg_gains_small - agg_losses_small);
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
