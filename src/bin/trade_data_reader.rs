/*
 *
 *  *
 *  *
 *  *
 *  *
 *  * MIT License
 *  * Copyright (c) 2023. Dwight J. Browne
 *  * dwight[-dot-]browne[-at-]dwightjbrowne[-dot-]com
 *  *
 *  *
 *  * Permission is hereby granted, free of charge, to any person obtaining a copy
 *  * of this software and associated documentation files (the "Software"), to deal
 *  * in the Software without restriction, including without limitation the rights
 *  * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *  * copies of the Software, and to permit persons to whom the Software is
 *  * furnished to do so, subject to the following conditions:
 *  *
 *  * The above copyright notice and this permission notice shall be included in all
 *  * copies or substantial portions of the Software.
 *  *
 *  * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *  * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  * SOFTWARE.
 *
 */

use dotenvy::dotenv;
use std::{env, io};
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
use feed_parser::nyse::mt220::{T220, Tc2, Tc4};
use feed_parser::nyse::base_funcs::{NYSEMsg, Stats};
use thousands::Separable;
use std::time::Instant;
use walkdir::WalkDir;
use chrono::prelude::*;

fn main() {
    dotenv().ok();
    let data_dir = env::var("NYSE_TRADE_DATA_DIR").expect("No Data file found!");
    for file in WalkDir::new(data_dir).into_iter().filter_map(|file| file.ok()) {
        if file.file_type().is_file() {
            process_file(file.path().display().to_string());
            println!("\n\n-----------------------------------\n\n");
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn process_file(data_file: String) {
    let start = Instant::now();
    println!("Processing file {}", data_file);
    let mut stats = Stats::new();
    let mut ctr = 0;
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(ip) = line {
                ctr += 1;
                let msg_type = process_line(ip, &mut stats);
                match msg_type {
                    Ok(()) => (),
                    Err(e) => {
                        println!("error processing line :{}", ctr);
                        println!("error: {}", e);
                        exit(1);
                    }
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Processed {} records in {} seconds", ctr.separate_with_commas(), duration.as_secs().separate_with_commas());
    println!("Symbol Index Mapping Messages {}", stats.msg_stats.msg_count[&NYSEMsg::T003].separate_with_commas());
    println!("Symbol Security Status Message {}", stats.msg_stats.msg_count[&NYSEMsg::T034].separate_with_commas());
    println!("Trade Messages {}", stats.msg_stats.msg_count[&NYSEMsg::T220].separate_with_commas());
    println!("Trade Message details: Number of symbols {}", stats.trade_stats.get_symbol_count().separate_with_commas());
    println!("Trade Message details: Trade Volume {}", stats.trade_stats.get_total_volume().separate_with_commas());
    println!("Trade Message details: average_rate {}/second ", stats.trade_stats.get_average_rate().separate_with_commas());
    println!("50 Most Active Symbols: {:?} ",stats.symbol_stats.get_most_active());
    println!("50 Highest Volume Symbols: {:?} ",stats.symbol_stats.get_highest_volume());
    println!("{} Activity: {:?}","TSLA",stats.event_stats.symbol_events.get("TSLA").unwrap().get_time_series());
}


fn process_line(line: String, stats: &mut Stats) -> Result<(), Box<dyn Error>> {
    //todo!(Need some error handling here)
    let toks: Vec<String> = line.split(',')
        .map(|s| s.to_string())
        .collect();

    let msg_type = NYSEMsg::get(&toks[0]);

    stats.msg_stats.add(msg_type);

    match msg_type {
      NYSEMsg::T034 => {
            Ok(())
        }
        NYSEMsg::T003 => {
            let  symbol = &toks[2];
            stats.event_stats.init(&symbol.clone());
            Ok(stats.symbol_stats.add(&symbol))
        }
        NYSEMsg::T220 => {
            match T220::new(toks) {
                Ok(trade) => {
                    if  trade.trade_cond4 == Tc4::OOpenPrice || trade.trade_cond4==Tc4::OClosePrice{
                        return std::result::Result::Ok(());
                    }
                    if  trade.trade_cond2 == Tc2::MCCT || trade.trade_cond2==Tc2::MCO{
                        return std::result::Result::Ok(());
                    }
                    _= stats.trade_stats.add(&trade);
                    // if trade.symbol =="BAC" {
                    //     println!("BAC: {:?}: {:?}, {:?}, {:?}, {:?}, {:?}, {:?} ", trade.source_time, trade.price, trade.volume, trade.trade_cond1, trade.trade_cond2,trade.trade_cond3,trade.trade_cond4     );
                    // }
                    stats.event_stats.update(&trade.symbol, &trade.source_time, &trade.price, trade.volume)?;
                    Ok(stats.symbol_stats.update(&trade.symbol, trade.volume))// todo! need to fix this
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        NYSEMsg::ERROR => {
            Err("Unknown message type".into())
        }
    }
}

