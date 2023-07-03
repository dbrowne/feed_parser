
pub mod dt {
    // see page 3 of https://www.nyse.com/publicdocs/nyse/data/TAQ_Pillar_Products_Client_Spec_v2.3i.pdf

    use std::collections::HashMap;

    pub struct Typ3 {
        //symbol Index Mapping Message
        pub msg_type: i32,
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

    pub struct Type34 {
        // Security Status Message
        pub msg_type: i32,
        pub seq_num: i32,
        pub source_time: String,        // really in HH:MM:SS.nnnnnnnnn format
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


    pub  enum T3MarketID {
        NYSE,
        NYSEArcaEq,
        NYSEArcaOpt,
        NYSEBonds,
        NYSEAmexOpt,
        NYSEAmerEq,
        NYSENatEq,
        NYSEChiEq,
    }

    pub struct T3MarketIDMap {
        map: HashMap<i32, T3MarketID>,
    }

    impl T3MarketIDMap {
        pub fn new() -> Self {
            let mut map = HashMap::new();
            map.insert(1, T3MarketID::NYSE);
            map.insert(3, T3MarketID::NYSEArcaEq);
            map.insert(3, T3MarketID::NYSEArcaOpt);
            map.insert(5, T3MarketID::NYSEBonds);
            map.insert(8, T3MarketID::NYSEAmexOpt);
            map.insert(9, T3MarketID::NYSEAmerEq);
            map.insert(10, T3MarketID::NYSENatEq);
            map.insert(11, T3MarketID::NYSEChiEq);
            Self { map }
        }

        pub fn get(&self, key: i32) -> Option<&T3MarketID> {
            self.map.get(&key)
        }
    }

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

    pub enum SSRTriggeringExchangeID {
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
        map: HashMap<char, SSRTriggeringExchangeID>,
    }

    impl SSRTExIDMap {
        pub fn new() -> Self {
            let mut map = HashMap::new();
            map.insert('N', SSRTriggeringExchangeID::NYSE);
            map.insert('P', SSRTriggeringExchangeID::NYSEArca);
            map.insert('C', SSRTriggeringExchangeID::NYSENational); // This is a conflict with 'C' - NSX
            map.insert('Q', SSRTriggeringExchangeID::NASDAQ);
            map.insert('A', SSRTriggeringExchangeID::NYSEAmerican);
            map.insert('B', SSRTriggeringExchangeID::NASDAQOMXBX);
            map.insert('D', SSRTriggeringExchangeID::FINRA);
            map.insert('I', SSRTriggeringExchangeID::ISE);
            map.insert('J', SSRTriggeringExchangeID::EDGA);
            map.insert('K', SSRTriggeringExchangeID::EDGX);
            map.insert('L', SSRTriggeringExchangeID::LTSE);
            map.insert('M', SSRTriggeringExchangeID::NYSEChicago);
            map.insert('S', SSRTriggeringExchangeID::CTS);
            map.insert('T', SSRTriggeringExchangeID::NASDAQOMX);
            map.insert('V', SSRTriggeringExchangeID::IEX);
            map.insert('W', SSRTriggeringExchangeID::CBSX);
            map.insert('X', SSRTriggeringExchangeID::NASDAQOMXPSX);
            map.insert('Y', SSRTriggeringExchangeID::CBOEBYX);
            map.insert('Z', SSRTriggeringExchangeID::CBOEBZX);
            Self { map }
        }

        pub fn get(&self, key: char) -> Option<&SSRTriggeringExchangeID> {
            self.map.get(&key)
        }
    }
}

