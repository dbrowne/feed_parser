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

#[derive(Eq, Hash, PartialEq, Debug)]
pub  enum NYSEMsg {
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
pub  struct MsgStats {
    msg_count: HashMap<NYSEMsg, i32>,

}


impl MsgStats {
    pub  fn new() -> MsgStats {
        MsgStats {
            msg_count: HashMap::new(),
        }
    }

    pub fn add(&mut self, msg: NYSEMsg) {
        let count = self.msg_count.entry(msg).or_insert(0);
        *count += 1;
    }
}

#[derive(Debug, Clone,PartialEq)]
pub struct TradeStats {
    count: HashMap<String, i32> ,
    avg_price: HashMap<String, f64>,
    total_volume: HashMap<String, i32>,

}

impl TradeStats {
    pub fn new() -> TradeStats {
        TradeStats {
            count: HashMap::new(),
            avg_price: HashMap::new(),
            total_volume: HashMap::new(),
        }
    }
    pub fn add(&mut self, symbol: &str, price: f64, volume: i32) {
        let count = self.count.entry(symbol.to_string()).or_insert(0);
        *count += 1;
        let total_volume = self.total_volume.entry(symbol.to_string()).or_insert(0);
        *total_volume += volume;
        let avg_price = self.avg_price.entry(symbol.to_string()).or_insert(0.0);
        *avg_price = (*avg_price * (*count as f64 - 1.0) + price) / (*count as f64);
    }
}


#[derive(Debug)]
pub  struct Stats{
    pub msg_stats: MsgStats,
    pub trade_stats: TradeStats,
}

impl Stats {
    pub  fn new() -> Stats {
        Stats {
            msg_stats: MsgStats::new(),
            trade_stats: TradeStats::new(),
        }
    }

}

#[cfg(test)]
mod  test {
    #[test]
    fn test_get_msg_type() {
        use super::get_msg_type;
        use super::NYSEMsg;
        assert_eq!(get_msg_type("3"), NYSEMsg::T003);
        assert_eq!(get_msg_type("34"), NYSEMsg::T034);
        assert_eq!(get_msg_type("202"), NYSEMsg::T220);
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
}