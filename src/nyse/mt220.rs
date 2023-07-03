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

pub struct T220 {
    //Trade Message
    pub msg_type: i8,
    pub seq_num: i32,
    pub source_time: String,
    // really in HH:MM:SS.nnnnnnnnn format
    pub symbol: String,
    pub symbol_seq_num: i32,
    pub trade_id: i32,
    pub price: f32,
    pub volume: i32,
    pub trade_cond1: char,
    pub trade_cond2: char,
    pub trade_cond3: char,
    pub trade_cond4: char,

}
#[derive(PartialEq, Debug)]
enum TC_1 {
    // • @ – Regular Sale (Arca, American, National, Chicago and NYSE)
    RegularSale,
    // • ‘C’ – Cash (TRF or Chicago only)
    Cash,
    // • ‘N’ – Next Day Trade (TRF or Chicago only)
    NextDayTrade,
    // • ‘ ’ – (space) Regular Sale (TRF only)
    RegularSaleTRF,
    // • ‘R’ – Seller (TRF only)
    Seller,
}

struct TC_1_Map {
    map: HashMap<char, TC_1>,
}

impl TC_1_Map {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('@', TC_1::RegularSale);
        map.insert('C', TC_1::Cash);
        map.insert('N', TC_1::NextDayTrade);
        map.insert(' ', TC_1::RegularSaleTRF);
        map.insert('R', TC_1::Seller);
        TC_1_Map { map }
    }
    pub fn get(&self, key: &char) -> Option<&TC_1> {
        self.map.get(key)
    }
}

#[derive(PartialEq, Debug)]
enum TC_2 {
    // ‘ ’ – N/A (0x20)
    NA,
    // • ‘F’ – Intermarket Sweep Order
    ISO,
    // • ‘O’ – Market Center Opening Trade
    MCO,
    // • ‘4’ – Derivatively priced (TRF only)
    DerivP,
    // • ‘5’ - Reopening Trade
    ReopeningTrade,
    // • ‘6’ – Market Center Closing Trade
    MCCT,
    // • ‘7’ – Qualified Contingent Trade (TRF or Chicago only)
    QCT,
    // • ‘9’ - Corrected Consolidated Close
    CCC,
}

struct TC_2_Map {
    map: HashMap<char, TC_2>,
}

impl TC_2_Map {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(' ', TC_2::NA);
        map.insert('F', TC_2::ISO);
        map.insert('O', TC_2::MCO);
        map.insert('4', TC_2::DerivP);
        map.insert('5', TC_2::ReopeningTrade);
        map.insert('6', TC_2::MCCT);
        map.insert('7', TC_2::QCT);
        map.insert('9', TC_2::CCC);
        TC_2_Map { map }
    }
    pub fn get(&self, key: &char) -> Option<&TC_2> {
        self.map.get(key)
    }
}
#[derive(PartialEq, Debug)]
enum TC_3 {
    // ‘ ’ – (space, or 0x20) N/A
    NA,
    // • ‘T’ – Extended Hours Trade
    ExtendedHoursTrade,
    // • ‘U’ – Extended Hours Sold (Out of Sequence)
    ExtendedHoursSold,
    // • ‘Z’ – Sold
    Sold,
}

struct TC_3_Map {
    map: HashMap<char, TC_3>,
}

impl TC_3_Map {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(' ', TC_3::NA);
        map.insert('T', TC_3::ExtendedHoursTrade);
        map.insert('U', TC_3::ExtendedHoursSold);
        map.insert('Z', TC_3::Sold);
        TC_3_Map { map }
    }
    pub fn get(&self, key: &char) -> Option<&TC_3> {
        self.map.get(key)
    }
}

#[derive(PartialEq, Debug)]
enum TC_4 {
    // • ‘ ’– (space, or 0x20) N/A
    NA,
    // • ‘I’ – Odd Lot Trade
    OddLotTrade,
    // • ‘M’ – Official Closing Price
    OClosePrice,
    // • ‘Q’ – Official Open Price
    OOpenPrice,
    // • ‘V’ – Contingent Trade
    ContTrade,
    // • ‘P’ – Prior Reference Price (TRF or Chicago only)
    PriorRefPrice,
    // • ‘W’ – Weighted Average Price (TRF only)
    WeightedAvgPrice,
}

struct TC_4_Map {
    map: HashMap<char, TC_4>,
}

impl TC_4_Map {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(' ', TC_4::NA);
        map.insert('I', TC_4::OddLotTrade);
        map.insert('M', TC_4::OClosePrice);
        map.insert('Q', TC_4::OOpenPrice);
        map.insert('V', TC_4::ContTrade);
        map.insert('P', TC_4::PriorRefPrice);
        map.insert('W', TC_4::WeightedAvgPrice);
        TC_4_Map { map }
    }
    pub fn get(&self, key: &char) -> Option<&TC_4> {
        self.map.get(key)
    }
}


#[cfg(test)]
mod  test{
    #[test]
    fn T_TC_1_Map(){
        use super::TC_1_Map;
        let tc_1_map = TC_1_Map::new();
        assert_eq!(tc_1_map.get(&'@').unwrap(), &super::TC_1::RegularSale);
        assert_eq!(tc_1_map.get(&'C').unwrap(), &super::TC_1::Cash);
        assert_eq!(tc_1_map.get(&'N').unwrap(), &super::TC_1::NextDayTrade);
        assert_eq!(tc_1_map.get(&' ').unwrap(), &super::TC_1::RegularSaleTRF);
        assert_eq!(tc_1_map.get(&'R').unwrap(), &super::TC_1::Seller);
    }

    #[test]
    fn T_TC_2_Map(){
        use  super::TC_2_Map;
        let tc_2_map = TC_2_Map::new();
        assert_eq!(tc_2_map.get(&' ').unwrap(), &super::TC_2::NA);
        assert_eq!(tc_2_map.get(&'F').unwrap(), &super::TC_2::ISO);
        assert_eq!(tc_2_map.get(&'O').unwrap(), &super::TC_2::MCO);
        assert_eq!(tc_2_map.get(&'4').unwrap(), &super::TC_2::DerivP);
        assert_eq!(tc_2_map.get(&'5').unwrap(), &super::TC_2::ReopeningTrade);
        assert_eq!(tc_2_map.get(&'6').unwrap(), &super::TC_2::MCCT);
        assert_eq!(tc_2_map.get(&'7').unwrap(), &super::TC_2::QCT);
        assert_eq!(tc_2_map.get(&'9').unwrap(), &super::TC_2::CCC);
    }

    #[test]
    fn T_TC_3_Map(){
        use  super::TC_3_Map;
        let tc_3_map = TC_3_Map::new();
        assert_eq!(tc_3_map.get(&' ').unwrap(), &super::TC_3::NA);
        assert_eq!(tc_3_map.get(&'T').unwrap(), &super::TC_3::ExtendedHoursTrade);
        assert_eq!(tc_3_map.get(&'U').unwrap(), &super::TC_3::ExtendedHoursSold);
        assert_eq!(tc_3_map.get(&'Z').unwrap(), &super::TC_3::Sold);
    }

    #[test]
    fn T_TC_4_Map(){
        use  super::TC_4_Map;
        let tc_4_map = TC_4_Map::new();
        assert_eq!(tc_4_map.get(&' ').unwrap(), &super::TC_4::NA);
        assert_eq!(tc_4_map.get(&'I').unwrap(), &super::TC_4::OddLotTrade);
        assert_eq!(tc_4_map.get(&'M').unwrap(), &super::TC_4::OClosePrice);
        assert_eq!(tc_4_map.get(&'Q').unwrap(), &super::TC_4::OOpenPrice);
        assert_eq!(tc_4_map.get(&'V').unwrap(), &super::TC_4::ContTrade);
        assert_eq!(tc_4_map.get(&'P').unwrap(), &super::TC_4::PriorRefPrice);
        assert_eq!(tc_4_map.get(&'W').unwrap(), &super::TC_4::WeightedAvgPrice);
    }
}


