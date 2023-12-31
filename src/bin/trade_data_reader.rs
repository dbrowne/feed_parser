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
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use feed_parser::nyse::base_funcs::{NYSEMsg, Stats};
use thousands::Separable;
use priority_queue::DoublePriorityQueue;
use walkdir::WalkDir;
use feed_parser::graphics::test_plot1::{test_plot_003, test_plot_004, test_power_spec_graph, test_spectral_density_graph};
use feed_parser::math_funcs::pre_processing::{diff_series, freq_counter, gen_price_with_fft, huff_code};
use feed_parser::general::parsing;
use feed_parser::event_structs::EventList;

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
    if let Ok(mut dta) = parsing::proc_file(&input_file) {
        evaluate_trades(&mut dta);
        dump_stats(&mut dta);

    }
}

fn dump_stats(stats: &mut Stats) {
    println!("Symbol Index Mapping Messages {}", stats.msg_stats.msg_count[&NYSEMsg::T003].separate_with_commas());
    println!("Symbol Security Status Message {}", stats.msg_stats.msg_count[&NYSEMsg::T034].separate_with_commas());
    println!("Trade Messages {}", stats.msg_stats.msg_count[&NYSEMsg::T220].separate_with_commas());
    println!("Trade Message details: Number of symbols {}", stats.trade_stats.get_symbol_count().separate_with_commas());
    println!("Trade Message details: Trade Volume {}", stats.trade_stats.get_total_volume().separate_with_commas());
    println!("Trade Message details: average_rate {}/second ", stats.trade_stats.get_average_rate().separate_with_commas());
    println!("50 Most Active Symbols: {:?} ", stats.symbol_stats.get_most_active(50)); //wish this was not mutable
    println!("50 Highest Volume Symbols: {:?} ", stats.symbol_stats.get_highest_volume(50));

    // println!("{} Activity: {:?}","TSLA",stats.event_stats.symbol_events.get("TSLA").unwrap().get_full_time_series());

    for (symbol, _) in stats.symbol_stats.get_most_active(50) {
        let event_list = stats.event_stats.symbol_events.get(&symbol).unwrap();
        _ = test_plot_003(&symbol, event_list.get_full_time_series_s(), event_list.get_min_max_price_volume());
        let fft_prices = gen_price_with_fft(&event_list.get_full_time_series_s());
        _ = test_plot_004(&symbol, fft_prices, event_list.get_min_max_price_volume());
        _ = test_power_spec_graph(&symbol, event_list.get_full_time_series_s(), event_list.get_max_tic_per_second());
        _ = test_spectral_density_graph(&symbol, event_list.get_full_time_series_s(), event_list.get_max_tic_per_second());
        extra_stats(event_list);

    }
}

fn  evaluate_trades(stats: &Stats){
    let  mut greatest_p_variance:DoublePriorityQueue<String, i32> = DoublePriorityQueue::new();
    let  mut freq_map:HashMap<String,HashMap<i32,u32>>= HashMap::new();

    for (symbol, event_list) in stats.event_stats.symbol_events.iter() {
        let  time_series = event_list.get_integer_time_series();
        let  pric_series = get_prices(time_series);
        if  pric_series.len() <3 {
            continue;
        }
        let diff_ser = diff_series(&pric_series);
        let freqs = freq_counter(diff_ser);
        let freq_size = freqs.keys().len() as i32;
        freq_map.insert(symbol.clone(), freqs);
        greatest_p_variance.push(symbol.clone(), freq_size);
    }

    loop{


        if let  Some((tkr,cnt)) = greatest_p_variance.pop_max() {
            println!("max variance for {} {}", tkr, cnt);
        }else{
            break;
        }
    }



}


fn  get_prices(inp:Vec<(String, i32,i32)>) ->Vec<i32>{
    let  mut outv:Vec<i32> = Vec::with_capacity(inp.len());

    for (_,p,_ )in inp  {
        outv.push(p.clone());
    }

    outv
}
fn  extra_stats(event_l:&EventList){
    let  int_series = event_l.get_integer_time_series();

    let  mut price_vec:Vec<i32> = Vec::with_capacity(int_series.len());
    for (_,price,_)  in int_series  {
        price_vec.push(price);

    }
    let  diffs:Vec<i32>=diff_series(&price_vec);
    let  freqs = freq_counter(diffs);
    // println!("frequencies {freqs:?}");
    huff_code(freqs);
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}







