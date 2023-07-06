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
    pub trade_cond1: Tc1,
    pub trade_cond2: Tc2,
    pub trade_cond3: Tc3,
    pub trade_cond4: Tc4,
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
            trade_cond1: Tc1::get(&inp[8]),
            trade_cond2: Tc2::get(&inp[9]),
            trade_cond3: Tc3::get(&inp[10]),
            trade_cond4: Tc4::get(&inp[11]),
        })
    }
}

#[derive(PartialEq, Debug, Hash)]
pub enum Tc1 {
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

impl Tc1 {
    pub fn get(inp: &str) -> Tc1 {
        match inp {
            "C" => Tc1::Cash,
            "N" => Tc1::NextDayTrade,
            " " => Tc1::RegularSaleTRF,
            "R" => Tc1::Seller,
            "@" => Tc1::RegularSale,
            _ => Tc1::Error,
        }
    }
}

#[derive(PartialEq, Debug, Hash)]
pub enum Tc2 {
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

impl Tc2 {
    pub fn get(inp: &str) -> Tc2 {
        match inp {
            " " => Tc2::NA,
            "F" => Tc2::ISO,
            "O" => Tc2::MCO,
            "4" => Tc2::DerivP,
            "5" => Tc2::ReopeningTrade,
            "6" => Tc2::MCCT,
            "7" => Tc2::QCT,
            "9" => Tc2::CCC,
            _ => Tc2::Error,
        }
    }
}

#[derive(PartialEq, Debug, Hash)]
pub enum Tc3 {
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

impl Tc3 {
    pub fn get(inp: &str) -> Tc3 {
        match inp {
            " " => Tc3::NA,
            "T" => Tc3::ExtendedHoursTrade,
            "U" => Tc3::ExtendedHoursSold,
            "Z" => Tc3::Sold,
            _ => Tc3::Error,
        }
    }
}

#[derive(PartialEq, Debug, Hash)]
pub enum Tc4 {
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

impl Tc4 {
    
pub fn get(inp: &str) -> Tc4 {
    match inp {
        " " => Tc4::NA,
        "I" => Tc4::OddLotTrade,
        "M" => Tc4::OClosePrice,
        "Q" => Tc4::OOpenPrice,
        "V" => Tc4::ContTrade,
        "P" => Tc4::PriorRefPrice,
        "W" => Tc4::WeightedAvgPrice,
        _ => Tc4::Error,
    }
}
}


#[cfg(test)]
mod test {
    use crate::nyse::mt220::*;

    #[test]
    fn test_get_tc4() {
        assert_eq!(Tc4::get(" "), Tc4::NA);
        assert_eq!(Tc4::get("I"), Tc4::OddLotTrade);
        assert_eq!(Tc4::get("M"), Tc4::OClosePrice);
        assert_eq!(Tc4::get("Q"), Tc4::OOpenPrice);
        assert_eq!(Tc4::get("V"), Tc4::ContTrade);
        assert_eq!(Tc4::get("P"), Tc4::PriorRefPrice);
        assert_eq!(Tc4::get("W"), Tc4::WeightedAvgPrice);
        assert_eq!(Tc4::get("A"), Tc4::Error);
    }

    #[test]
    fn test_tc1() {
        assert_eq!(Tc1::get("@"), Tc1::RegularSale);
        assert_eq!(Tc1::get("C"), Tc1::Cash);
        assert_eq!(Tc1::get("N"), Tc1::NextDayTrade);
        assert_eq!(Tc1::get(" "), Tc1::RegularSaleTRF);
        assert_eq!(Tc1::get("R"), Tc1::Seller);
        assert_eq!(Tc1::get(""), Tc1::Error);
    }

    #[test]
    fn test_tc2() {
        assert_eq!(Tc2::get(" "), Tc2::NA);
        assert_eq!(Tc2::get("F"), Tc2::ISO);
        assert_eq!(Tc2::get("O"), Tc2::MCO);
        assert_eq!(Tc2::get("4"), Tc2::DerivP);
        assert_eq!(Tc2::get("5"), Tc2::ReopeningTrade);
        assert_eq!(Tc2::get("6"), Tc2::MCCT);
        assert_eq!(Tc2::get("7"), Tc2::QCT);
        assert_eq!(Tc2::get("9"), Tc2::CCC);
        assert_eq!(Tc2::get("A"), Tc2::Error);
    }

    #[test]
    fn test_tc3get() {
        assert_eq!(Tc3::get(" "), Tc3::NA);
        assert_eq!(Tc3::get("T"), Tc3::ExtendedHoursTrade);
        assert_eq!(Tc3::get("U"), Tc3::ExtendedHoursSold);
        assert_eq!(Tc3::get("Z"), Tc3::Sold);
        assert_eq!(Tc3::get("A"), Tc3::Error);
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
            trade_cond1: Tc1::RegularSale,
            trade_cond2: Tc2::ISO,
            trade_cond3: Tc3::ExtendedHoursTrade,
            trade_cond4: Tc4::OddLotTrade,
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





