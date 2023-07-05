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




use std::collections::HashMap;
use std::error::Error;
use crate::nyse::mt220::T220;
use crate::time_funcs::time_to_dec;

#[derive(Eq, Hash, PartialEq, Debug,Copy, Clone)]
pub enum NYSEMsg {
    T003,
    T034,
    T220,
    ERROR,
}


pub fn get_msg_type(msg: &str) -> NYSEMsg {
    match msg {
        "3" => NYSEMsg::T003,
        "34" => NYSEMsg::T034,
        "220" => NYSEMsg::T220,
        _ => NYSEMsg::ERROR
    }
}

#[derive(Debug)]
pub struct MsgStats {
    msg_count: HashMap<NYSEMsg, i32>,

}


impl MsgStats {
    pub fn new() -> MsgStats {
        MsgStats {
            msg_count: HashMap::new(),
        }
    }

    pub fn add(&mut self, msg: NYSEMsg) {
        let count = self.msg_count.entry(msg).or_insert(0);
        *count += 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradeStats {
    symbols: HashMap<String, i32>,
    rate: HashMap<i32, i32>,
    // second and count per second
    total_volume: HashMap<String, i32>,
}

impl TradeStats {
    pub fn new() -> TradeStats {
        TradeStats {
            symbols: HashMap::new(),
            rate: HashMap::new(),
            total_volume: HashMap::new(),
        }
    }

    pub fn add(&mut self, trade: &T220) -> Result<(), Box<dyn Error>> {
        let second = time_to_dec(&trade.source_time.as_str())?.round() as i32;
        let symbol = trade.symbol.clone();

        let count = self.symbols.entry(symbol.clone()).or_insert(0);
        *count += 1;

        let volume = self.total_volume.entry(symbol).or_insert(0);
        *volume += trade.volume;

        let rate_count = self.rate.entry(second).or_insert(0);
        *rate_count += 1;

        Ok(())
    }

    pub fn get_symbol_count(&self) -> i32 {
        self.symbols.len() as i32
    }

    pub fn get_volume_per_symbol(&self, symbol: &str) -> i32 {
        match self.total_volume.get(symbol) {
            Some(count) => *count,
            None => 0,
        }
    }

    pub fn get_total_volume(&self) -> i64 {
        let mut total: i64 = 0;
        for (_, volume) in &self.total_volume {
            total += *volume as i64;
        }
        total
    }

    pub fn get_average_rate(&self) -> f32 {
        let count = self.rate.len() as i32;
        if count == 0 {
            return 0.0;
        }
        let mut rate_sum: i64 = 0;
        for (_, rate) in &self.rate {
            rate_sum += *rate as i64;
        }
        rate_sum as f32 / count as f32
    }
}


#[derive(Debug)]
pub struct Stats {
    pub msg_stats: MsgStats,
    pub trade_stats: TradeStats,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            msg_stats: MsgStats::new(),
            trade_stats: TradeStats::new(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_msg_type() {
        use super::get_msg_type;
        use super::NYSEMsg;
        assert_eq!(get_msg_type("3"), NYSEMsg::T003);
        assert_eq!(get_msg_type("34"), NYSEMsg::T034);
        assert_eq!(get_msg_type("220"), NYSEMsg::T220);
        assert_eq!(get_msg_type("0"), NYSEMsg::ERROR);
    }

    #[test]
    fn test_msg_stats() {
        use super::MsgStats;
        use super::NYSEMsg;
        let mut stats = MsgStats::new();
        stats.add(NYSEMsg::T003);
        stats.add(NYSEMsg::T003);
        stats.add(NYSEMsg::T034);
        stats.add(NYSEMsg::T034);
        stats.add(NYSEMsg::T034);
        stats.add(NYSEMsg::T220);
        stats.add(NYSEMsg::T220);
        stats.add(NYSEMsg::T220);
        stats.add(NYSEMsg::T220);
        stats.add(NYSEMsg::T220);
        stats.add(NYSEMsg::T220);
        stats.add(NYSEMsg::T220);
        assert_eq!(stats.msg_count.get(&NYSEMsg::T003).unwrap(), &2);
        assert_eq!(stats.msg_count.get(&NYSEMsg::T034).unwrap(), &3);
        assert_eq!(stats.msg_count.get(&NYSEMsg::T220).unwrap(), &7);
    }

    #[test]
    fn test_trade_stats_symbol() {
        use super::TradeStats;
        use super::T220;
        let mut stats = TradeStats::new();
        let trade_vec = vec!["220".to_string(),
                             "12345".to_string(),
                             "09:30:01.00005090000".to_string(),
                             "IBM".to_string(),
                             "100".to_string(),
                             "1001".to_string(),
                             "99.95".to_string(),
                             "1000".to_string(),
                             "@".to_string(),
                             "F".to_string(),
                             "T".to_string(),
                             "I".to_string()];
        let trade = T220::new(trade_vec).unwrap();
        stats.add(&trade).unwrap();

        assert_eq!(stats.get_symbol_count(), 1);

        let trade_vec = vec!["220".to_string(),
                             "12345".to_string(),
                             "09:30:01.00005090000".to_string(),
                             "AAPL".to_string(),
                             "100".to_string(),
                             "1001".to_string(),
                             "99.95".to_string(),
                             "1000".to_string(),
                             "@".to_string(),
                             "F".to_string(),
                             "T".to_string(),
                             "I".to_string()];

        stats.add(&T220::new(trade_vec).unwrap()).unwrap();

        assert_eq!(stats.get_symbol_count(), 2);

        let trade_vec = vec!["220".to_string(),
                             "12345".to_string(),
                             "09:30:01.00005090000".to_string(),
                             "MSFT".to_string(),
                             "100".to_string(),
                             "1001".to_string(),
                             "99.95".to_string(),
                             "1000".to_string(),
                             "@".to_string(),
                             "F".to_string(),
                             "T".to_string(),
                             "I".to_string()];

        stats.add(&T220::new(trade_vec).unwrap()).unwrap();
        assert_eq!(stats.get_symbol_count(), 3);

        let trade_vec = vec!["220".to_string(),
                             "12345".to_string(),
                             "09:30:01.00005090000".to_string(),
                             "MSFT".to_string(),
                             "100".to_string(),
                             "1001".to_string(),
                             "99.95".to_string(),
                             "1500".to_string(),
                             "@".to_string(),
                             "F".to_string(),
                             "T".to_string(),
                             "I".to_string()];

        stats.add(&T220::new(trade_vec).unwrap()).unwrap();
        assert_eq!(stats.get_symbol_count(), 3);

        assert_eq!(stats.get_volume_per_symbol("IBM"), 1000);
        assert_eq!(stats.get_volume_per_symbol("MSFT"), 2500);
        assert_eq!(stats.get_total_volume(), 4500);
        assert_eq!(stats.get_average_rate().round(), 4.0);

    }
}