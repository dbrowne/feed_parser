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


use std::collections::HashMap;


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
}

pub struct SSRTExIDMap {
    map: HashMap<char, SSRTexID>,
}

impl SSRTExIDMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('N', SSRTexID::NYSE);
        map.insert('P', SSRTexID::NYSEArca);
        map.insert('C', SSRTexID::NYSENational); // This is a conflict with 'C' - NSX
        map.insert('Q', SSRTexID::NASDAQ);
        map.insert('A', SSRTexID::NYSEAmerican);
        map.insert('B', SSRTexID::NASDAQOMXBX);
        map.insert('D', SSRTexID::FINRA);
        map.insert('I', SSRTexID::ISE);
        map.insert('J', SSRTexID::EDGA);
        map.insert('K', SSRTexID::EDGX);
        map.insert('L', SSRTexID::LTSE);
        map.insert('M', SSRTexID::NYSEChicago);
        map.insert('S', SSRTexID::CTS);
        map.insert('T', SSRTexID::NASDAQOMX);
        map.insert('V', SSRTexID::IEX);
        map.insert('W', SSRTexID::CBSX);
        map.insert('X', SSRTexID::NASDAQOMXPSX);
        map.insert('Y', SSRTexID::CBOEBYX);
        map.insert('Z', SSRTexID::CBOEBZX);
        Self { map }
    }

    pub fn get(&self, key: char) -> Option<&SSRTexID> {
        self.map.get(&key)
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
}

pub struct SecurityStatusMap {
    map: HashMap<char, SecurityStatus>,
}

impl SecurityStatusMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('4', SecurityStatus::Halt);
        map.insert('5', SecurityStatus::Resume);
        map.insert('A', SecurityStatus::SSRA);
        map.insert('C', SecurityStatus::SSRC);
        map.insert('D', SecurityStatus::SSRD);
        map.insert('P', SecurityStatus::PreO);
        map.insert('B', SecurityStatus::Beg);
        map.insert('E', SecurityStatus::Early);
        map.insert('O', SecurityStatus::Core);
        map.insert('L', SecurityStatus::Late);
        map.insert('X', SecurityStatus::Closed);
        map.insert('I', SecurityStatus::PI);
        map.insert('G', SecurityStatus::PreOPI);
        Self { map }
    }

    pub fn get(&self, key: char) -> Option<&SecurityStatus> {
        self.map.get(&key)
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
}

pub struct HaltConditionMap {
    map: HashMap<char, HaltCondition>,
}

impl HaltConditionMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('~', HaltCondition::NotDelayed);
        map.insert('D', HaltCondition::NewsRel);
        map.insert('I', HaltCondition::OrdImb);
        map.insert('P', HaltCondition::NewsPend);
        map.insert('M', HaltCondition::LULDPause);
        map.insert('X', HaltCondition::EquipChange);
        map.insert('Z', HaltCondition::NOOpNoRes);
        map.insert('A', HaltCondition::AddlInfReq);
        map.insert('C', HaltCondition::RegCon);
        map.insert('E', HaltCondition::MergE);
        map.insert('F', HaltCondition::ETFMisPr);
        map.insert('N', HaltCondition::CorpA);
        map.insert('O', HaltCondition::NewOff);
        map.insert('V', HaltCondition::NoIntraDay);
        map.insert('1', HaltCondition::HaltL1);
        map.insert('2', HaltCondition::HaltL2);
        map.insert('3', HaltCondition::HaltL3);
        Self { map }
    }

    pub fn get(&self, key: char) -> Option<&HaltCondition> {
        self.map.get(&key)
    }
}

#[derive(PartialEq, Debug)]
pub  enum SSRState {
    // ‘~’ – No Short Sale in Effect
    NoSSR,
    // • ‘E’ – Short Sale Restriction in Effect
    SSR,
}

pub struct SSRStateMap {
    map: HashMap<char, SSRState>,
}

impl SSRStateMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('~', SSRState::NoSSR);
        map.insert('E', SSRState::SSR);
        Self { map }
    }

    pub fn get(&self, key: char) -> Option<&SSRState> {
        self.map.get(&key)
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
}


pub struct MarketStateMap {
    map: HashMap<char, MarketState>,
}

impl MarketStateMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('P', MarketState::PreOp);
        map.insert('E', MarketState::EarlySess);
        map.insert('O', MarketState::CoreSEss);
        map.insert('L', MarketState::LateSess);
        map.insert('X', MarketState::Closed);
        Self { map }
    }

    pub fn get(&self, key: char) -> Option<&MarketState> {
        self.map.get(&key)
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn t_ssrtex_idmap() {
        let map = super::SSRTExIDMap::new();
        assert_eq!(map.get('N'), Some(&super::SSRTexID::NYSE));
        assert_eq!(map.get('P'), Some(&super::SSRTexID::NYSEArca));
        assert_eq!(map.get('C'), Some(&super::SSRTexID::NYSENational));
        assert_eq!(map.get('Q'), Some(&super::SSRTexID::NASDAQ));
        assert_eq!(map.get('A'), Some(&super::SSRTexID::NYSEAmerican));
        assert_eq!(map.get('B'), Some(&super::SSRTexID::NASDAQOMXBX));
        assert_eq!(map.get('D'), Some(&super::SSRTexID::FINRA));
        assert_eq!(map.get('I'), Some(&super::SSRTexID::ISE));
        assert_eq!(map.get('J'), Some(&super::SSRTexID::EDGA));
        assert_eq!(map.get('K'), Some(&super::SSRTexID::EDGX));
        assert_eq!(map.get('L'), Some(&super::SSRTexID::LTSE));
        assert_eq!(map.get('M'), Some(&super::SSRTexID::NYSEChicago));
        assert_eq!(map.get('S'), Some(&super::SSRTexID::CTS));
        assert_eq!(map.get('T'), Some(&super::SSRTexID::NASDAQOMX));
        assert_eq!(map.get('V'), Some(&super::SSRTexID::IEX));
        assert_eq!(map.get('W'), Some(&super::SSRTexID::CBSX));
        assert_eq!(map.get('X'), Some(&super::SSRTexID::NASDAQOMXPSX));
        assert_eq!(map.get('Y'), Some(&super::SSRTexID::CBOEBYX));
        assert_eq!(map.get('Z'), Some(&super::SSRTexID::CBOEBZX));
        assert_eq!(map.get('F'), None);
    }

    #[test]
    fn t_security_status_map() {
        let map = super::SecurityStatusMap::new();
        assert_eq!(map.get('4'), Some(&super::SecurityStatus::Halt));
        assert_eq!(map.get('5'), Some(&super::SecurityStatus::Resume));
        assert_eq!(map.get('A'), Some(&super::SecurityStatus::SSRA));
        assert_eq!(map.get('C'), Some(&super::SecurityStatus::SSRC));
        assert_eq!(map.get('C'), Some(&super::SecurityStatus::SSRC));
        assert_eq!(map.get('D'), Some(&super::SecurityStatus::SSRD));
        assert_eq!(map.get('P'), Some(&super::SecurityStatus::PreO));
        assert_eq!(map.get('B'), Some(&super::SecurityStatus::Beg));
        assert_eq!(map.get('E'), Some(&super::SecurityStatus::Early));
        assert_eq!(map.get('O'), Some(&super::SecurityStatus::Core));
        assert_eq!(map.get('L'), Some(&super::SecurityStatus::Late));
        assert_eq!(map.get('X'), Some(&super::SecurityStatus::Closed));
        assert_eq!(map.get('I'), Some(&super::SecurityStatus::PI));
        assert_eq!(map.get('G'), Some(&super::SecurityStatus::PreOPI));
        assert_eq!(map.get('H'), None);
    }

    #[test]
    fn t_halt_condidtion_map() {
        let map = super::HaltConditionMap::new();
        assert_eq!(map.get('~'), Some(&super::HaltCondition::NotDelayed));
        assert_eq!(map.get('D'), Some(&super::HaltCondition::NewsRel));
        assert_eq!(map.get('I'), Some(&super::HaltCondition::OrdImb));
        assert_eq!(map.get('P'), Some(&super::HaltCondition::NewsPend));
        assert_eq!(map.get('M'), Some(&super::HaltCondition::LULDPause));
        assert_eq!(map.get('X'), Some(&super::HaltCondition::EquipChange));
        assert_eq!(map.get('Z'), Some(&super::HaltCondition::NOOpNoRes));
        assert_eq!(map.get('A'), Some(&super::HaltCondition::AddlInfReq));
        assert_eq!(map.get('C'), Some(&super::HaltCondition::RegCon));
        assert_eq!(map.get('E'), Some(&super::HaltCondition::MergE));
        assert_eq!(map.get('F'), Some(&super::HaltCondition::ETFMisPr));
        assert_eq!(map.get('N'), Some(&super::HaltCondition::CorpA));
        assert_eq!(map.get('O'), Some(&super::HaltCondition::NewOff));
        assert_eq!(map.get('V'), Some(&super::HaltCondition::NoIntraDay));
        assert_eq!(map.get('1'), Some(&super::HaltCondition::HaltL1));
        assert_eq!(map.get('2'), Some(&super::HaltCondition::HaltL2));
        assert_eq!(map.get('3'), Some(&super::HaltCondition::HaltL3));
        assert_eq!(map.get('$'), None);
    }

    #[test]
    fn t_ssrstate_map() {
        let map = super::SSRStateMap::new();
        assert_eq!(map.get('~'), Some(&super::SSRState::NoSSR));
        assert_eq!(map.get('E'), Some(&super::SSRState::SSR));
        assert_eq!(map.get(' '), None);
    }

    #[test]
    fn t_NarketStateNap() {
        let map = super::MarketStateMap::new();
        assert_eq!(map.get('P'), Some(&super::MarketState::PreOp));
        assert_eq!(map.get('E'), Some(&super::MarketState::EarlySess));
        assert_eq!(map.get('O'), Some(&super::MarketState::CoreSEss));
        assert_eq!(map.get('L'), Some(&super::MarketState::LateSess));
        assert_eq!(map.get('X'), Some(&super::MarketState::Closed));
        assert_eq!(map.get('A'), None);
    }
}


