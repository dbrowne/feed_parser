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

use std::collections::HashMap;

pub struct T3 {
    //symbol Index Mapping Message
    pub msg_type: i8,
    pub seq_num: i32,
    pub symbol: String,
    pub market_id: i32,
    pub system_id: i32,
    pub exchange_code: String,
    pub security_type: String,
    pub lot_size: i32,
    pub prev_close_price: f32,
    pub prev_close_volume: i32,
    pub price_resolution: i32,
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
}


pub struct MarketIDMap {
    map: HashMap<i8, MarketID>,
}

impl MarketIDMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(1, MarketID::NYSE);
        map.insert(3, MarketID::NYSEArcaEq);
        map.insert(4, MarketID::NYSEArcaOpt);
        map.insert(5, MarketID::NYSEBonds);
        map.insert(8, MarketID::NYSEAmexOpt);
        map.insert(9, MarketID::NYSEAmerEq);
        map.insert(10, MarketID::NYSENatEq);
        map.insert(11, MarketID::NYSEChiEq);
        Self { map }
    }

    pub fn get(&self, key: i8) -> Option<&MarketID> {
        self.map.get(&key)
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
}

pub struct SecurityTypeMap {
    map: HashMap<char, SecuryType>,
}

impl SecurityTypeMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('A', SecuryType::ADR);
        map.insert('C', SecuryType::ComStk);
        map.insert('D', SecuryType::Deben);
        map.insert('E', SecuryType::ETF);
        map.insert('F', SecuryType::Foreign);
        map.insert('H', SecuryType::ADShares);
        map.insert('I', SecuryType::Units);
        map.insert('L', SecuryType::IdxLnkdNotes);
        map.insert('M', SecuryType::OtherBlank);
        map.insert('O', SecuryType::OrdShrs);
        map.insert('P', SecuryType::Pfd);
        map.insert('R', SecuryType::Rights);
        map.insert('S', SecuryType::SoBenInt);
        map.insert('T', SecuryType::Test);
        map.insert('U', SecuryType::CEF);
        map.insert('X', SecuryType::IdxSec);
        map.insert('Y', SecuryType::War);
        Self { map }
    }

    pub fn get(&self, key: char) -> Option<&SecuryType> {
        self.map.get(&key)
    }
}

#[derive(PartialEq, Debug)]
pub  enum PriceResolution {
        AllPenny,
        PennyNickel,
        NickelDime,
}

pub struct PriceResolutionMap {
    map: HashMap<i8, PriceResolution>,
}

impl PriceResolutionMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(0, PriceResolution::AllPenny);
        map.insert(1, PriceResolution::PennyNickel);
        map.insert(5, PriceResolution::NickelDime);
        Self { map }
    }

    pub fn get(&self, key: i8) -> Option<&PriceResolution> {
        self.map.get(&key)
    }
}




#[cfg(test)]
mod  test{
    #[test]
    fn test_MarketIDMap() {
        use super::*;
        let market_id_map = MarketIDMap::new();
        assert_eq!(market_id_map.get(1), Some(&MarketID::NYSE));
        assert_eq!(market_id_map.get(3), Some(&MarketID::NYSEArcaEq));
        assert_eq!(market_id_map.get(4), Some(&MarketID::NYSEArcaOpt));
        assert_eq!(market_id_map.get(5), Some(&MarketID::NYSEBonds));
        assert_eq!(market_id_map.get(8), Some(&MarketID::NYSEAmexOpt));
        assert_eq!(market_id_map.get(9), Some(&MarketID::NYSEAmerEq));
        assert_eq!(market_id_map.get(10), Some(&MarketID::NYSENatEq));
        assert_eq!(market_id_map.get(11), Some(&MarketID::NYSEChiEq));
    }


    #[test]
    fn test_SecurityTypeMap() {
        use super::*;
        let security_type_map = SecurityTypeMap::new();
        assert_eq!(security_type_map.get('A'), Some(&SecuryType::ADR));
        assert_eq!(security_type_map.get('C'), Some(&SecuryType::ComStk));
        assert_eq!(security_type_map.get('D'), Some(&SecuryType::Deben));
        assert_eq!(security_type_map.get('E'), Some(&SecuryType::ETF));
        assert_eq!(security_type_map.get('F'), Some(&SecuryType::Foreign));
        assert_eq!(security_type_map.get('H'), Some(&SecuryType::ADShares));
        assert_eq!(security_type_map.get('I'), Some(&SecuryType::Units));
        assert_eq!(security_type_map.get('L'), Some(&SecuryType::IdxLnkdNotes));
        assert_eq!(security_type_map.get('M'), Some(&SecuryType::OtherBlank));
        assert_eq!(security_type_map.get('O'), Some(&SecuryType::OrdShrs));
        assert_eq!(security_type_map.get('P'), Some(&SecuryType::Pfd));
        assert_eq!(security_type_map.get('R'), Some(&SecuryType::Rights));
        assert_eq!(security_type_map.get('S'), Some(&SecuryType::SoBenInt));
        assert_eq!(security_type_map.get('T'), Some(&SecuryType::Test));
        assert_eq!(security_type_map.get('U'), Some(&SecuryType::CEF));
        assert_eq!(security_type_map.get('X'), Some(&SecuryType::IdxSec));
        assert_eq!(security_type_map.get('Y'), Some(&SecuryType::War));
    }
    #[test]
    fn test_PriceResolutionMap() {
        use super::*;
        let price_resolution_map = PriceResolutionMap::new();
        assert_eq!(price_resolution_map.get(0), Some(&PriceResolution::AllPenny));
        assert_eq!(price_resolution_map.get(1), Some(&PriceResolution::PennyNickel));
        assert_eq!(price_resolution_map.get(5), Some(&PriceResolution::NickelDime));
    }

}