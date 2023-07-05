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


pub fn get_market_id(id: &str) -> MarketID {
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

#[derive(PartialEq, Debug)]
pub enum SecuryType {
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


pub fn get_security_type(id: &str) -> SecuryType {
    match id {
        "A" => SecuryType::ADR,
        "C" => SecuryType::ComStk,
        "D" => SecuryType::Deben,
        "E" => SecuryType::ETF,
        "F" => SecuryType::Foreign,
        "H" => SecuryType::ADShares,
        "I" => SecuryType::Units,
        "L" => SecuryType::IdxLnkdNotes,
        "M" => SecuryType::OtherBlank,
        "O" => SecuryType::OrdShrs,
        "P" => SecuryType::Pfd,
        "R" => SecuryType::Rights,
        "S" => SecuryType::SoBenInt,
        "T" => SecuryType::Test,
        "U" => SecuryType::CEF,
        "X" => SecuryType::IdxSec,
        "Y" => SecuryType::War,
        _ => SecuryType::ERROR
    }
}


#[derive(PartialEq, Debug)]
pub enum PriceResolution {
    AllPenny,
    PennyNickel,
    NickelDime,
    ERROR,
}

pub fn get_price_resolution(id: &str) -> PriceResolution {
    match id {
        "0" => PriceResolution::AllPenny,
        "1" => PriceResolution::PennyNickel,
        "5" => PriceResolution::NickelDime,
        _ => PriceResolution::ERROR,
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_market_id() {
        use super::*;
        assert_eq!(get_market_id("1"), MarketID::NYSE);
        assert_eq!(get_market_id("3"), MarketID::NYSEArcaEq);
        assert_eq!(get_market_id("4"), MarketID::NYSEArcaOpt);
        assert_eq!(get_market_id("5"), MarketID::NYSEBonds);
        assert_eq!(get_market_id("8"), MarketID::NYSEAmexOpt);
        assert_eq!(get_market_id("9"), MarketID::NYSEAmerEq);
        assert_eq!(get_market_id("10"), MarketID::NYSENatEq);
        assert_eq!(get_market_id("11"), MarketID::NYSEChiEq);
        assert_eq!(get_market_id("12"), MarketID::ERROR);
    }


    #[test]
    fn test_security_type() {
        use super::*;
        assert_eq!(get_security_type("A"), SecuryType::ADR);
        assert_eq!(get_security_type("C"), SecuryType::ComStk);
        assert_eq!(get_security_type("D"), SecuryType::Deben);
        assert_eq!(get_security_type("E"), SecuryType::ETF);
        assert_eq!(get_security_type("F"), SecuryType::Foreign);
        assert_eq!(get_security_type("H"), SecuryType::ADShares);
        assert_eq!(get_security_type("I"), SecuryType::Units);
        assert_eq!(get_security_type("L"), SecuryType::IdxLnkdNotes);
        assert_eq!(get_security_type("M"), SecuryType::OtherBlank);
        assert_eq!(get_security_type("O"), SecuryType::OrdShrs);
        assert_eq!(get_security_type("P"), SecuryType::Pfd);
        assert_eq!(get_security_type("R"), SecuryType::Rights);
        assert_eq!(get_security_type("S"), SecuryType::SoBenInt);
        assert_eq!(get_security_type("T"), SecuryType::Test);
        assert_eq!(get_security_type("U"), SecuryType::CEF);
        assert_eq!(get_security_type("X"), SecuryType::IdxSec);
        assert_eq!(get_security_type("Y"), SecuryType::War);
        assert_eq!(get_security_type("Z"), SecuryType::ERROR);
    }

    #[test]
    fn test_price_resolution() {
        use super::*;
        assert_eq!(get_price_resolution("0"), PriceResolution::AllPenny);
        assert_eq!(get_price_resolution("1"), PriceResolution::PennyNickel);
        assert_eq!(get_price_resolution("5"), PriceResolution::NickelDime);
        assert_eq!(get_price_resolution("6"), PriceResolution::ERROR);
    }
}