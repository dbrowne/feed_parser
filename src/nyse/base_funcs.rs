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
use priority_queue::PriorityQueue;
use crate::event_structs::EventList;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum NYSEMsg {
    T003,
    T034,
    T220,
    ERROR,
}

impl NYSEMsg {
    pub fn get(msg: &str) -> NYSEMsg {
        match msg {
            "3" => NYSEMsg::T003,
            "34" => NYSEMsg::T034,
            "220" => NYSEMsg::T220,
            _ => NYSEMsg::ERROR
        }
    }
}


#[derive(Debug)]
pub struct MsgStats {
    pub msg_count: HashMap<NYSEMsg, i32>,

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
pub struct EventStats {
   pub symbol_events: HashMap<String, EventList>,
}

impl EventStats {
    pub fn new() -> EventStats {
        EventStats {
            symbol_events: HashMap::new(),
        }
    }
    pub fn init(&mut self, symbol: &str) {
        let event_list = EventList::new();
        self.symbol_events.insert(symbol.to_string(), event_list);
    }
    pub fn update(&mut self, symbol: &str, seconds:&str, s_price:&str, volume:i32 ) -> Result<(), Box<dyn Error>>{

        match self.symbol_events.get_mut(symbol) {
            Some(event_list) => {
                let f_second = time_to_dec(seconds)?;
                let price: f32 = s_price.parse::<f32>()?;
                event_list.update(f_second, price, volume);
            }
            None => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Symbol not found")));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradeStats {
    symbols: HashMap<String, i32>,
    rate: HashMap<i32, i32>,
    // second and count per second
    symbol_volume: HashMap<String, i32>,
    total_volume: i64,

}

impl TradeStats {
    pub fn new() -> TradeStats {
        TradeStats {
            symbols: HashMap::new(),
            rate: HashMap::new(),
            symbol_volume: HashMap::new(),
            total_volume: 0,
        }
    }


    pub fn add(&mut self, trade: &T220) -> Result<(), Box<dyn Error>> {
        let second = time_to_dec(&trade.source_time.as_str())?.round() as i32;
        let symbol = trade.symbol.clone();

        let count = self.symbols.entry(symbol.clone()).or_insert(0);
        *count += 1;

        let volume = self.symbol_volume.entry(symbol).or_insert(0);
        *volume += trade.volume;
        self.total_volume += trade.volume as i64;
        let rate_count = self.rate.entry(second).or_insert(0);
        *rate_count += 1;


        Ok(())
    }

    pub fn get_symbol_count(&self) -> i32 {
        self.symbols.len() as i32
    }

    pub fn get_count_per_symbol(&self, symbol: &str) -> i32 {
        match self.symbol_volume.get(symbol) {
            Some(count) => *count,
            None => 0,
        }
    }

    pub fn get_total_volume(&self) -> i64 {
        self.total_volume
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

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolStats {
    most_active: PriorityQueue<String, i32>,
    highest_volume: PriorityQueue<String, i32>,
    active_hash: HashMap<String, i32>,
    active_volume: HashMap<String, i32>,
    symbol_count: i32,

}

impl SymbolStats {
    pub fn new() -> SymbolStats {
        SymbolStats {
            most_active: PriorityQueue::new(),
            highest_volume: PriorityQueue::new(),
            active_hash: HashMap::new(),
            active_volume: HashMap::new(),
            symbol_count: 0,
        }
    }

    pub fn get_symbol_count(&self) -> i32 {
        self.symbol_count
    }

    pub fn add(&mut self, symbol: &str) {
        self.most_active.push(symbol.to_string(), 0);
        self.highest_volume.push(symbol.to_string(), 0);
        self.active_hash.insert(symbol.to_string(), 0);
        self.active_volume.insert(symbol.to_string(), 0);
        self.symbol_count += 1;
    }
    pub fn update(&mut self, symbol: &str, volume: i32) {
        let mut active_count = self.active_hash.entry(symbol.to_string()).or_insert(0);
        *active_count += 1;

        let mut active_volume = self.active_volume.entry(symbol.to_string()).or_insert(0);
        *active_volume += volume;

        self.most_active.change_priority(&symbol.to_string(), *active_count);
        self.highest_volume.change_priority(&symbol.to_string(), *active_volume);
    }

    pub fn get_most_active(&mut self) -> Vec<(String, i32)> {
        let mut symbols: Vec<(String, i32)> = Vec::new();
        let mut ctr = 0;

        while !self.most_active.is_empty() {
            let (symbol, volume) = self.most_active.pop().unwrap();
            symbols.push((symbol.clone(), volume.clone()));
            ctr += 1;
            if ctr > 50 {
                break;
            }
        }
        symbols
    }

    pub fn get_highest_volume(&mut self) -> Vec<(String, i32)> {
        let mut symbols: Vec<(String, i32)> = Vec::new();
        let mut ctr = 0;

        while !self.highest_volume.is_empty() {
            let (symbol, volume) = self.highest_volume.pop().unwrap();
            symbols.push((symbol.clone(), volume.clone()));
            ctr += 1;
            if ctr > 50 {
                break;
            }
        }
        symbols
    }
}


#[derive(Debug)]
pub struct Stats {
    pub msg_stats: MsgStats,
    pub trade_stats: TradeStats,
    pub symbol_stats: SymbolStats,
    pub  event_stats: EventStats,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            msg_stats: MsgStats::new(),
            trade_stats: TradeStats::new(),
            symbol_stats: SymbolStats::new(),
            event_stats: EventStats::new(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_msg_type() {
        use super::NYSEMsg;
        assert_eq!(NYSEMsg::get("3"), NYSEMsg::T003);
        assert_eq!(NYSEMsg::get("34"), NYSEMsg::T034);
        assert_eq!(NYSEMsg::get("220"), NYSEMsg::T220);
        assert_eq!(NYSEMsg::get("0"), NYSEMsg::ERROR);
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

        assert_eq!(stats.get_count_per_symbol("IBM"), 1000);
        assert_eq!(stats.get_count_per_symbol("MSFT"), 2500);
        assert_eq!(stats.get_total_volume(), 4500);
        assert_eq!(stats.get_average_rate().round(), 4.0);
    }

    #[test]
    fn test_symbol_stats_add() {
        use super::SymbolStats;
        let mut stats = SymbolStats::new();
        stats.add("IBM");
        stats.add("AAPL");
        stats.add("MSFT");
        assert_eq!(stats.get_symbol_count(), 3);
    }

    #[test]
    fn test_symbol_stats_add_volume() {
        use super::SymbolStats;
        let mut stats = SymbolStats::new();
        stats.add("IBM");
        stats.add("AAPL");
        stats.add("MSFT");
        stats.update("IBM", 1000);
        stats.update("AAPL", 2000);
        stats.update("AAPL", 2000);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);

        assert_eq!(stats.get_highest_volume()[0], ("AAPL".to_string(), 4000));
    }

    #[test]
    fn test_symbol_stat_add_active() {
        use super::SymbolStats;
        let mut stats = SymbolStats::new();
        stats.add("IBM");
        stats.add("AAPL");
        stats.add("MSFT");
        stats.update("IBM", 1000);
        stats.update("AAPL", 2000);
        stats.update("AAPL", 2000);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        stats.update("IBM", 10);
        assert_eq!(stats.get_most_active()[0], ("IBM".to_string(), 7));
    }
}