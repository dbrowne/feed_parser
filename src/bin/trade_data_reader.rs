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
use feed_parser::graphics::test_plot1::{test_plot_003, test_plot_004, test_power_spec_graph, test_spectral_density_graph};
use feed_parser::math_funcs::pre_processing::gen_price_with_fft;
use feed_parser::general::parsing;


fn main() {
    dotenv().ok();
    let data_dir = env::var("NYSE_TRADE_DATA_DIR").expect("No Data file found!");
    for file in WalkDir::new(data_dir).into_iter().filter_map(|file| file.ok()) {
        if file.file_type().is_file() {
            proc_dta(file.path().display().to_string());
            println!("\n\n-----------------------------------\n\n");
        }
    }
}


fn proc_dta(input_file: String) {
    let start = Instant::now();
    if let Ok(mut dta) = parsing::proc_file(&input_file) {
        dump_stats(&mut dta);
        let duration = start.elapsed();
    }
}

fn dump_stats(stats: &mut Stats) {
    println!("Symbol Index Mapping Messages {}", stats.msg_stats.msg_count[&NYSEMsg::T003].separate_with_commas());
    println!("Symbol Security Status Message {}", stats.msg_stats.msg_count[&NYSEMsg::T034].separate_with_commas());
    println!("Trade Messages {}", stats.msg_stats.msg_count[&NYSEMsg::T220].separate_with_commas());
    println!("Trade Message details: Number of symbols {}", stats.trade_stats.get_symbol_count().separate_with_commas());
    println!("Trade Message details: Trade Volume {}", stats.trade_stats.get_total_volume().separate_with_commas());
    println!("Trade Message details: average_rate {}/second ", stats.trade_stats.get_average_rate().separate_with_commas());
    println!("50 Most Active Symbols: {:?} ", stats.symbol_stats.get_most_active(50));
    println!("50 Highest Volume Symbols: {:?} ", stats.symbol_stats.get_highest_volume(50));
    // println!("{} Activity: {:?}","TSLA",stats.event_stats.symbol_events.get("TSLA").unwrap().get_full_time_series());

    for (symbol, _) in stats.symbol_stats.get_most_active() {
        let event_list = stats.event_stats.symbol_events.get(&symbol).unwrap();
        _ = test_plot_003(&symbol, event_list.get_full_time_series_s(), event_list.get_min_max_price_volume());
        let fft_prices = gen_price_with_fft(&event_list.get_full_time_series_s());
        _ = test_plot_004(&symbol, fft_prices, event_list.get_min_max_price_volume());
        _ = test_power_spec_graph(&symbol, event_list.get_full_time_series_s(), event_list.get_max_tic_per_second());
        _ = test_spectral_density_graph(&symbol, event_list.get_full_time_series_s(), event_list.get_max_tic_per_second());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}







