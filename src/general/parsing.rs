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

use indicatif::ProgressBar;
use instant::Instant;
use std::io::{self,BufRead};
use crate::nyse::mt220::{T220, Tc2, Tc4};
use crate::nyse::base_funcs::{NYSEMsg, Stats};
use std::path::Path;
use std::fs::File;
use std::process::exit;
use std::error::Error;

const MSG_IDX: usize = 0;
const SYMBOL_IDX: usize = 2;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn proc_file(data_file: &str) -> Result<Stats, Box<dyn std::error::Error>> {
    let start: Instant = Instant::now();
    println!("Processing file: {}", data_file);
    let bar: ProgressBar = ProgressBar::new(5_000_000);
    let mut stats: Stats = Stats::new();
    let mut ctr: i32 = 0;
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(msg) = line {
                ctr += 1;
                let msg_type = process_line(msg, &mut stats);
                match msg_type {
                    Ok(()) => (),

                    Err(e) => {
                        println!("Error processing line: {}", e);
                        println!("error: {}", e);
                        bar.finish_with_message("Failed processing");
                        exit(1);
                    }
                }
                bar.inc(1);
            }
        }
    }
    bar.finish_with_message("done");
    Result::Ok(stats)
}


fn process_line(line: String, stats: &mut Stats) -> Result<(), Box<dyn Error>> {
    let tokens: Vec<String> = line.split(',')
        .map(|s| s.to_string())
        .collect();

    let msg_type: NYSEMsg = NYSEMsg::get(&tokens[MSG_IDX]);
    stats.msg_stats.add(msg_type);

    match msg_type {
        NYSEMsg::T003 => {
            let symbol: &String = &tokens[SYMBOL_IDX];
            stats.event_stats.init(&symbol.clone());
            Ok(stats.symbol_stats.add(&symbol))
        }
        NYSEMsg::T034 => {
            Ok(())
        }
        NYSEMsg::T220 => {
            match T220::new(tokens) {
                Ok(trade) => {
                    if  trade.trade_cond4 == Tc4::OOpenPrice || trade.trade_cond4 == Tc4::OClosePrice{
                        return std::result::Result::Ok(());
                    }
                    if  trade.trade_cond2 == Tc2::MCCT || trade.trade_cond2==Tc2::MCO{
                        return std::result::Result::Ok(());
                    }
                    _= stats.trade_stats.add(&trade);
                    stats.event_stats.update(&trade.symbol, &trade.source_time, &trade.price, trade.volume)?;
                    Ok(stats.symbol_stats.update(&trade.symbol, trade.volume))
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        NYSEMsg::ERROR => {
            Err("unknown message type".into())
        }
    }
}