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


// Security Status Message Type 34
// Based on https://www.nyse.com/publicdocs/nyse/data/TAQ_Pillar_Products_Client_Spec_v2.3i.pdf



pub struct T34 {
    // Security Status Message
    pub msg_type: u8,
    pub seq_num: i32,
    pub source_time: String,
    // really in HH:MM:SS.nnnnnnnnn format
    pub symbol: String,
    pub symbol_seq_num: i32,
    pub sec_status: String,
    pub halt_condition: String,
    pub price_1: f32,
    pub price_2: f32,
    pub ssr_t_exid: String,
    pub ssr_t_vol: i32,
    pub time: String,
    // assuming in HH:MM:SS.nnnnnnnnn format
    pub market_state: String,
}




#[derive(PartialEq, Debug)]
pub enum SSRTexID {
    // spec maps C top both NSX and NYSE on pag 16 Dropping NSX
    NYSE,
    NYSEArca,
    NYSENational,
    NASDAQ,
    NYSEAmerican,
    NASDAQOMXBX,
    FINRA,
    ISE,
    EDGA,
    EDGX,
    LTSE,
    NYSEChicago,
    CTS,
    NASDAQOMX,
    IEX,
    CBSX,
    NASDAQOMXPSX,
    CBOEBYX,
    CBOEBZX,
    ERROR
}

impl SSRTexID {
    pub  fn  get(id :&str) -> SSRTexID {
        match   id{
            "N" => SSRTexID::NYSE,
            "P" => SSRTexID::NYSEArca,
            "C" => SSRTexID::NYSENational,
            "Q" => SSRTexID::NASDAQ,
            "A" => SSRTexID::NYSEAmerican,
            "B" => SSRTexID::NASDAQOMXBX,
            "D" => SSRTexID::FINRA,
            "I" => SSRTexID::ISE,
            "J" => SSRTexID::EDGA,
            "K" => SSRTexID::EDGX,
            "L" => SSRTexID::LTSE,
            "M" => SSRTexID::NYSEChicago,
            "S" => SSRTexID::CTS,
            "T" => SSRTexID::NASDAQOMX,
            "V" => SSRTexID::IEX,
            "W" => SSRTexID::CBSX,
            "X" => SSRTexID::NASDAQOMXPSX,
            "Y" => SSRTexID::CBOEBYX,
            "Z" => SSRTexID::CBOEBZX,
            _ => SSRTexID::ERROR,
        }
    }
}




#[derive(PartialEq, Debug)]
pub enum SecurityStatus {
    // Trading Halt
    Halt,
    Resume,
    // – Short Sale Restriction Activated (Day 1)
    SSRA,
    // - Short Sale Restriction Continued (Day 2)
    SSRC,
    // - Short Sale Restriction Deactivated
    SSRD,
    // P – Pre-opening
    PreO,
    // • B - Begin accepting orders
    Beg,
    // • E – Early session
    Early,
    // • O – Core session
    Core,
    // • L – Late session
    Late,
    // • X – Closed
    Closed,
    //I – Price Indication
    PI,
    // • G – Pre-Opening Price Indication
    PreOPI,
    ERROR,
}

impl SecurityStatus {
    pub fn  get(id:&str) -> SecurityStatus {
        match  id {
            "4" => SecurityStatus::Halt,
            "5" => SecurityStatus::Resume,
            "A" => SecurityStatus::SSRA,
            "C" => SecurityStatus::SSRC,
            "D" => SecurityStatus::SSRD,
            "P" => SecurityStatus::PreO,
            "B" => SecurityStatus::Beg,
            "E" => SecurityStatus::Early,
            "O" => SecurityStatus::Core,
            "L" => SecurityStatus::Late,
            "X" => SecurityStatus::Closed,
            "I" => SecurityStatus::PI,
            "G" => SecurityStatus::PreOPI,
            _ => SecurityStatus::ERROR,
        }

    }
}



#[derive(PartialEq, Debug)]
pub enum HaltCondition {
    // • '~' - Security not delayed/halted
    NotDelayed,
    // • 'D' - News released
    NewsRel,
    // • 'I' - Order imbalance
    OrdImb,
    // • 'P' - News pending
    NewsPend,
    // • 'M' – LULD pause
    LULDPause,
    // • 'X' - Equipment changeover
    EquipChange,
    // • 'Z' - No open/No resume
    NOOpNoRes,
    // • A - Additional Information Requested
    AddlInfReq,
    // • C - Regulatory Concern
    RegCon,
    // • E - Merger Effective
    MergE,
    // • F - ETF Component Prices Not Available
    ETFMisPr,
    // • N - Corporate Action
    CorpA,
    // • O - New Security Offering
    NewOff,
    // • V - Intraday Indicative Value Not Available
    NoIntraDay,
    // • '1' - Market Wide Circuit Breaker Halt Level 1
    HaltL1,
    // • '2' - Market Wide Circuit Breaker Halt Level 2
    HaltL2,
    // • '3' - Market Wide Circuit Breaker Halt Level 3
    HaltL3,
    ERROR,
}

impl HaltCondition {
    pub  fn  get(id: &str) ->HaltCondition{
        match  id {
            "~" => HaltCondition::NotDelayed,
            "D" => HaltCondition::NewsRel,
            "I" => HaltCondition::OrdImb,
            "P" => HaltCondition::NewsPend,
            "M" => HaltCondition::LULDPause,
            "X" => HaltCondition::EquipChange,
            "Z" => HaltCondition::NOOpNoRes,
            "A" => HaltCondition::AddlInfReq,
            "C" => HaltCondition::RegCon,
            "E" => HaltCondition::MergE,
            "F" => HaltCondition::ETFMisPr,
            "N" => HaltCondition::CorpA,
            "O" => HaltCondition::NewOff,
            "V" => HaltCondition::NoIntraDay,
            "1" => HaltCondition::HaltL1,
            "2" => HaltCondition::HaltL2,
            "3" => HaltCondition::HaltL3,
            _ => HaltCondition::ERROR,
        }
    }
}



#[derive(PartialEq, Debug)]
pub  enum SSRState {
    // ‘~’ – No Short Sale in Effect
    NoSSR,
    // • ‘E’ – Short Sale Restriction in Effect
    SSR,
    ERROR,
}

impl SSRState {
    pub fn get(id: &str) -> SSRState {
        match id {
            "~" => SSRState::NoSSR,
            "E" => SSRState::SSR,
            _ => SSRState::ERROR,
        }
    }
    
}



#[derive(PartialEq, Debug)]
pub enum MarketState {
    // ‘P’ – Pre-opening
    PreOp,
    // • ‘E’ – Early session
    EarlySess,
    // • ‘O’ – Core session
    CoreSEss,
    // • ‘L’ – Late session (Non-NYSE only)
    LateSess,
    // • ‘X’ – Closed
    Closed,
    ERROR,
}

impl MarketState {
    pub  fn get(id: &str) ->MarketState{
        match id {
            "P" => MarketState::PreOp,
            "E" => MarketState::EarlySess,
            "O" => MarketState::CoreSEss,
            "L" => MarketState::LateSess,
            "X" => MarketState::Closed,
            _ => MarketState::ERROR,
        }
    }
    
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ssrtexid() {
        assert_eq!(SSRTexID::get("N"), SSRTexID::NYSE);
        assert_eq!(SSRTexID::get("P"), SSRTexID::NYSEArca);
        assert_eq!(SSRTexID::get("C"), SSRTexID::NYSENational);
        assert_eq!(SSRTexID::get("Q"), SSRTexID::NASDAQ);
        assert_eq!(SSRTexID::get("A"), SSRTexID::NYSEAmerican);
        assert_eq!(SSRTexID::get("B"), SSRTexID::NASDAQOMXBX);
        assert_eq!(SSRTexID::get("D"), SSRTexID::FINRA);
        assert_eq!(SSRTexID::get("I"), SSRTexID::ISE);
        assert_eq!(SSRTexID::get("J"), SSRTexID::EDGA);
        assert_eq!(SSRTexID::get("K"), SSRTexID::EDGX);
        assert_eq!(SSRTexID::get("L"), SSRTexID::LTSE);
        assert_eq!(SSRTexID::get("M"), SSRTexID::NYSEChicago);
        assert_eq!(SSRTexID::get("S"), SSRTexID::CTS);
        assert_eq!(SSRTexID::get("T"), SSRTexID::NASDAQOMX);
        assert_eq!(SSRTexID::get("V"), SSRTexID::IEX);
        assert_eq!(SSRTexID::get("W"), SSRTexID::CBSX);
        assert_eq!(SSRTexID::get("X"), SSRTexID::NASDAQOMXPSX);
        assert_eq!(SSRTexID::get("Y"), SSRTexID::CBOEBYX);
        assert_eq!(SSRTexID::get("Z"), SSRTexID::CBOEBZX);
        assert_eq!(SSRTexID::get("F"), SSRTexID::ERROR);
    }

    #[test]
    fn t_security_status() {
        assert_eq!(SecurityStatus::get("4"), SecurityStatus::Halt);
        assert_eq!(SecurityStatus::get("5"), SecurityStatus::Resume);
        assert_eq!(SecurityStatus::get("A"), SecurityStatus::SSRA);
        assert_eq!(SecurityStatus::get("C"), SecurityStatus::SSRC);
        assert_eq!(SecurityStatus::get("C"), SecurityStatus::SSRC);
        assert_eq!(SecurityStatus::get("D"), SecurityStatus::SSRD);
        assert_eq!(SecurityStatus::get("P"), SecurityStatus::PreO);
        assert_eq!(SecurityStatus::get("B"), SecurityStatus::Beg);
        assert_eq!(SecurityStatus::get("E"), SecurityStatus::Early);
        assert_eq!(SecurityStatus::get("O"), SecurityStatus::Core);
        assert_eq!(SecurityStatus::get("L"), SecurityStatus::Late);
        assert_eq!(SecurityStatus::get("X"), SecurityStatus::Closed);
        assert_eq!(SecurityStatus::get("I"), SecurityStatus::PI);
        assert_eq!(SecurityStatus::get("G"), SecurityStatus::PreOPI);
        assert_eq!(SecurityStatus::get("H"), SecurityStatus::ERROR);
    }

    #[test]
    fn t_halt_condition() {
        assert_eq!(HaltCondition::get("~"), HaltCondition::NotDelayed);
        assert_eq!(HaltCondition::get("D"), HaltCondition::NewsRel);
        assert_eq!(HaltCondition::get("I"), HaltCondition::OrdImb);
        assert_eq!(HaltCondition::get("P"), HaltCondition::NewsPend);
        assert_eq!(HaltCondition::get("M"), HaltCondition::LULDPause);
        assert_eq!(HaltCondition::get("X"), HaltCondition::EquipChange);
        assert_eq!(HaltCondition::get("Z"), HaltCondition::NOOpNoRes);
        assert_eq!(HaltCondition::get("A"), HaltCondition::AddlInfReq);
        assert_eq!(HaltCondition::get("C"), HaltCondition::RegCon);
        assert_eq!(HaltCondition::get("E"), HaltCondition::MergE);
        assert_eq!(HaltCondition::get("F"), HaltCondition::ETFMisPr);
        assert_eq!(HaltCondition::get("N"), HaltCondition::CorpA);
        assert_eq!(HaltCondition::get("O"), HaltCondition::NewOff);
        assert_eq!(HaltCondition::get("V"), HaltCondition::NoIntraDay);
        assert_eq!(HaltCondition::get("1"), HaltCondition::HaltL1);
        assert_eq!(HaltCondition::get("2"), HaltCondition::HaltL2);
        assert_eq!(HaltCondition::get("3"), HaltCondition::HaltL3);
        assert_eq!(HaltCondition::get("$"), HaltCondition::ERROR);
    }

    #[test]
    fn t_ssrstate() {
        assert_eq!(SSRState::get("~"), SSRState::NoSSR);
        assert_eq!(SSRState::get("E"), SSRState::SSR);
        assert_eq!(SSRState::get(" "), SSRState::ERROR);
    }

    #[test]
    fn t_market_state() {
        assert_eq!(MarketState::get("P"), MarketState::PreOp);
        assert_eq!(MarketState::get("E"), MarketState::EarlySess);
        assert_eq!(MarketState::get("O"), MarketState::CoreSEss);
        assert_eq!(MarketState::get("L"), MarketState::LateSess);
        assert_eq!(MarketState::get("X"), MarketState::Closed);
        assert_eq!(MarketState::get("A"), MarketState::ERROR);
    }
}


