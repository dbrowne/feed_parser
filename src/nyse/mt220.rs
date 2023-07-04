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
use crate::time_funcs::time_to_dec;
use std::error::Error;

#[derive(Debug, Hash, PartialEq)]
pub struct T220 {
    //Trade Message
    pub msg_type: u8,
    pub seq_num: i32,
    pub source_time: String,
    // really in HH:MM:SS.nnnnnnnnn format
    pub symbol: String,
    pub symbol_seq_num: i32,
    pub trade_id: i32,
    pub price: String,
    //needs to be string to be hashable :(
    pub volume: i32,
    pub trade_cond1: TC_1,
    pub trade_cond2: TC_2,
    pub trade_cond3: TC_3,
    pub trade_cond4: TC_4,
}


impl T220 {
    pub fn new(inp: Vec<String>) -> Result<Self, Box<dyn Error>> {
        Ok(T220 {
            msg_type: 220,
            seq_num: inp[1].parse::<i32>()?,
            source_time: inp[2].clone(),
            symbol: inp[3].clone(),
            symbol_seq_num: inp[4].parse::<i32>()?,
            trade_id: inp[5].parse::<i32>()?,
            price: inp[6].clone(),
            volume: inp[7].parse::<i32>()?,
            trade_cond1: get_tc_1(&inp[8]),
            trade_cond2: get_tc_2(&inp[9]),
            trade_cond3: get_tc_3(&inp[10]),
            trade_cond4: get_tc_4(&inp[11]),
        })
    }
}

#[derive(PartialEq, Debug, Hash)]
pub enum TC_1 {
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
    Error,
}

pub fn get_tc_1(inp: &str) -> TC_1 {
    match inp {
        "C" => TC_1::Cash,
        "N" => TC_1::NextDayTrade,
        " " => TC_1::RegularSaleTRF,
        "R" => TC_1::Seller,
        "@" => TC_1::RegularSale,
        _ => TC_1::Error,
    }
}


#[derive(PartialEq, Debug, Hash)]
pub enum TC_2 {
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
    Error,
}

pub fn get_tc_2(inp: &str) -> TC_2 {
    match inp {
        " " => TC_2::NA,
        "F" => TC_2::ISO,
        "O" => TC_2::MCO,
        "4" => TC_2::DerivP,
        "5" => TC_2::ReopeningTrade,
        "6" => TC_2::MCCT,
        "7" => TC_2::QCT,
        "9" => TC_2::CCC,
        _ => TC_2::Error,
    }
}


#[derive(PartialEq, Debug, Hash)]
pub enum TC_3 {
    // ‘ ’ – (space, or 0x20) N/A
    NA,
    // • ‘T’ – Extended Hours Trade
    ExtendedHoursTrade,
    // • ‘U’ – Extended Hours Sold (Out of Sequence)
    ExtendedHoursSold,
    // • ‘Z’ – Sold
    Sold,
    Error,
}

pub fn get_tc_3(inp: &str) -> TC_3 {
    match inp {
        " " => TC_3::NA,
        "T" => TC_3::ExtendedHoursTrade,
        "U" => TC_3::ExtendedHoursSold,
        "Z" => TC_3::Sold,
        _ => TC_3::Error,
    }
}

#[derive(PartialEq, Debug, Hash)]
pub enum TC_4 {
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
    Error,
}

pub fn get_tc_4(inp: &str) -> TC_4 {
    match inp {
        " " => TC_4::NA,
        "I" => TC_4::OddLotTrade,
        "M" => TC_4::OClosePrice,
        "Q" => TC_4::OOpenPrice,
        "V" => TC_4::ContTrade,
        "P" => TC_4::PriorRefPrice,
        "W" => TC_4::WeightedAvgPrice,
        _ => TC_4::Error,
    }
}


#[cfg(test)]
mod test {
    use crate::nyse::base_funcs::NYSEMsg::T220;
    use crate::nyse::mt220::*;

    #[test]
    fn test_get_tc4() {
        assert_eq!(get_tc_4(" "), TC_4::NA);
        assert_eq!(get_tc_4("I"), TC_4::OddLotTrade);
        assert_eq!(get_tc_4("M"), TC_4::OClosePrice);
        assert_eq!(get_tc_4("Q"), TC_4::OOpenPrice);
        assert_eq!(get_tc_4("V"), TC_4::ContTrade);
        assert_eq!(get_tc_4("P"), TC_4::PriorRefPrice);
        assert_eq!(get_tc_4("W"), TC_4::WeightedAvgPrice);
        assert_eq!(get_tc_4("A"), TC_4::Error);
    }

    #[test]
    fn test_get_tc_1() {
        assert_eq!(get_tc_1("@"), TC_1::RegularSale);
        assert_eq!(get_tc_1("C"), TC_1::Cash);
        assert_eq!(get_tc_1("N"), TC_1::NextDayTrade);
        assert_eq!(get_tc_1(" "), TC_1::RegularSaleTRF);
        assert_eq!(get_tc_1("R"), TC_1::Seller);
        assert_eq!(get_tc_1(""), TC_1::Error);
    }

    #[test]
    fn test_get_tc_2() {
        assert_eq!(get_tc_2(" "), TC_2::NA);
        assert_eq!(get_tc_2("F"), TC_2::ISO);
        assert_eq!(get_tc_2("O"), TC_2::MCO);
        assert_eq!(get_tc_2("4"), TC_2::DerivP);
        assert_eq!(get_tc_2("5"), TC_2::ReopeningTrade);
        assert_eq!(get_tc_2("6"), TC_2::MCCT);
        assert_eq!(get_tc_2("7"), TC_2::QCT);
        assert_eq!(get_tc_2("9"), TC_2::CCC);
        assert_eq!(get_tc_2("A"), TC_2::Error);
    }

    #[test]
    fn test_get_tc_3() {
        assert_eq!(get_tc_3(" "), TC_3::NA);
        assert_eq!(get_tc_3("T"), TC_3::ExtendedHoursTrade);
        assert_eq!(get_tc_3("U"), TC_3::ExtendedHoursSold);
        assert_eq!(get_tc_3("Z"), TC_3::Sold);
        assert_eq!(get_tc_3("A"), TC_3::Error);
    }

    #[test]
    fn test_t220() {
        use crate::nyse::mt220::T220;
        let a = T220 {
            msg_type: 220,
            seq_num: 12345,
            source_time: "09:30:01.00005090000".to_string(),
            symbol: "IBM".to_string(),
            symbol_seq_num: 100,
            trade_id: 1001,
            price: "99.95".to_string(),
            volume: 1000,
            trade_cond1: TC_1::RegularSale,
            trade_cond2: TC_2::ISO,
            trade_cond3: TC_3::ExtendedHoursTrade,
            trade_cond4: TC_4::OddLotTrade,
        };


        let b = vec!["220".to_string(),
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
        let c = T220::new(b).unwrap();
        assert_eq!(c, a);
    }
}





