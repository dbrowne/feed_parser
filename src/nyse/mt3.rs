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


// Based on  https://www.nyse.com/publicdocs/nyse/data/TAQ_Pillar_Products_Client_Spec_v2.3i.pdf
// Symbol Index Mapping Message (Type 3)



pub struct T3 {
    //symbol Index Mapping Message
    pub msg_type: u8,
    pub seq_num: i32,
    pub symbol: String,
    pub market_id: String,
    pub system_id: i32,
    pub exchange_code: String,
    pub security_type: String,
    pub lot_size: i32,
    pub prev_close_price: f32,
    pub prev_close_volume: i32,
    pub price_resolution: String,
    pub round_lot: String,
    pub mpv: f32,
    pub unit_of_trade: i32,
}


#[derive(PartialEq, Debug)]
pub enum MarketID {
    NYSE,
    NYSEArcaEq,
    NYSEArcaOpt,
    NYSEBonds,
    NYSEAmexOpt,
    NYSEAmerEq,
    NYSENatEq,
    NYSEChiEq,
    ERROR,
}

impl MarketID {
    pub fn get(id: &str) -> MarketID {
        match id {
            "1" => MarketID::NYSE,
            "3" => MarketID::NYSEArcaEq,
            "4" => MarketID::NYSEArcaOpt,
            "5" => MarketID::NYSEBonds,
            "8" => MarketID::NYSEAmexOpt,
            "9" => MarketID::NYSEAmerEq,
            "10" => MarketID::NYSENatEq,
            "11" => MarketID::NYSEChiEq,
            _ => MarketID::ERROR,
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum SecurityType {
    ADR,
    ComStk,
    Deben,
    ETF,
    Foreign,
    ADShares,
    Units,
    IdxLnkdNotes,
    OtherBlank,
    OrdShrs,
    Pfd,
    Rights,
    SoBenInt,
    Test,
    CEF,
    IdxSec,
    War,
    ERROR,
}

impl SecurityType {
    pub fn get(id: &str) -> SecurityType {
        match id {
            "A" => SecurityType::ADR,
            "C" => SecurityType::ComStk,
            "D" => SecurityType::Deben,
            "E" => SecurityType::ETF,
            "F" => SecurityType::Foreign,
            "H" => SecurityType::ADShares,
            "I" => SecurityType::Units,
            "L" => SecurityType::IdxLnkdNotes,
            "M" => SecurityType::OtherBlank,
            "O" => SecurityType::OrdShrs,
            "P" => SecurityType::Pfd,
            "R" => SecurityType::Rights,
            "S" => SecurityType::SoBenInt,
            "T" => SecurityType::Test,
            "U" => SecurityType::CEF,
            "X" => SecurityType::IdxSec,
            "Y" => SecurityType::War,
            _ => SecurityType::ERROR
        }
    }
}



#[derive(PartialEq, Debug)]
pub enum PriceResolution {
    AllPenny,
    PennyNickel,
    NickelDime,
    ERROR,
}

impl PriceResolution{
    pub fn get(id: &str) -> PriceResolution {
        match id {
            "0" => PriceResolution::AllPenny,
            "1" => PriceResolution::PennyNickel,
            "5" => PriceResolution::NickelDime,
            _ => PriceResolution::ERROR,
        }
    }
}



#[cfg(test)]
mod test {
    #[test]
    fn test_market_id() {
        use super::*;
        assert_eq!(MarketID::get("1"), MarketID::NYSE);
        assert_eq!(MarketID::get("3"), MarketID::NYSEArcaEq);
        assert_eq!(MarketID::get("4"), MarketID::NYSEArcaOpt);
        assert_eq!(MarketID::get("5"), MarketID::NYSEBonds);
        assert_eq!(MarketID::get("8"), MarketID::NYSEAmexOpt);
        assert_eq!(MarketID::get("9"), MarketID::NYSEAmerEq);
        assert_eq!(MarketID::get("10"), MarketID::NYSENatEq);
        assert_eq!(MarketID::get("11"), MarketID::NYSEChiEq);
        assert_eq!(MarketID::get("12"), MarketID::ERROR);
    }


    #[test]
    fn test_security_type() {
        use super::*;
        assert_eq!(SecurityType::get("A"), SecurityType::ADR);
        assert_eq!(SecurityType::get("C"), SecurityType::ComStk);
        assert_eq!(SecurityType::get("D"), SecurityType::Deben);
        assert_eq!(SecurityType::get("E"), SecurityType::ETF);
        assert_eq!(SecurityType::get("F"), SecurityType::Foreign);
        assert_eq!(SecurityType::get("H"), SecurityType::ADShares);
        assert_eq!(SecurityType::get("I"), SecurityType::Units);
        assert_eq!(SecurityType::get("L"), SecurityType::IdxLnkdNotes);
        assert_eq!(SecurityType::get("M"), SecurityType::OtherBlank);
        assert_eq!(SecurityType::get("O"), SecurityType::OrdShrs);
        assert_eq!(SecurityType::get("P"), SecurityType::Pfd);
        assert_eq!(SecurityType::get("R"), SecurityType::Rights);
        assert_eq!(SecurityType::get("S"), SecurityType::SoBenInt);
        assert_eq!(SecurityType::get("T"), SecurityType::Test);
        assert_eq!(SecurityType::get("U"), SecurityType::CEF);
        assert_eq!(SecurityType::get("X"), SecurityType::IdxSec);
        assert_eq!(SecurityType::get("Y"), SecurityType::War);
        assert_eq!(SecurityType::get("Z"), SecurityType::ERROR);
    }

    #[test]
    fn test_price_resolution() {
        use super::*;
        assert_eq!(PriceResolution::get("0"), PriceResolution::AllPenny);
        assert_eq!(PriceResolution::get("1"), PriceResolution::PennyNickel);
        assert_eq!(PriceResolution::get("5"), PriceResolution::NickelDime);
        assert_eq!(PriceResolution::get("6"), PriceResolution::ERROR);
    }
}