use crate::Common::{self, ProgramStatusType};
use crate::Qot_Common::{
    self, DarkStatus, ExchType, PlateSetType, QotMarket, SecurityStatus, SecurityType,
};
use protobuf::Enum;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct SecurityStaticBasic {
    pub security: Security,
    pub id: i64,
    pub lot_size: i32,
    pub sec_type: SecurityType,
    pub name: String,
    pub list_time: String,
    pub delisting: Option<bool>,
    pub list_timestamp: Option<f64>,
    pub exch_type: Option<ExchType>,
}

impl From<Qot_Common::SecurityStaticBasic> for SecurityStaticBasic {
    fn from(security_static_basic: Qot_Common::SecurityStaticBasic) -> Self {
        SecurityStaticBasic {
            security: security_static_basic.security.to_owned().unwrap().into(),
            id: security_static_basic.id(),
            lot_size: security_static_basic.lotSize(),
            sec_type: SecurityType::from_i32(security_static_basic.secType()).unwrap(),
            name: security_static_basic.name().into(),
            list_time: security_static_basic.listTime().into(),
            delisting: security_static_basic.delisting,
            list_timestamp: security_static_basic.listTimestamp,
            exch_type: ExchType::from_i32(security_static_basic.exchType()),
        }
    }
}

#[derive(Debug)]
pub struct SecurityStaticInfo {
    pub basic: SecurityStaticBasic,
}

pub struct PlateInfo {
    pub plate: Security,
    pub name: String,
    pub plate_type: Option<PlateSetType>,
}

impl From<Qot_Common::PlateInfo> for PlateInfo {
    fn from(plate_info: Qot_Common::PlateInfo) -> Self {
        PlateInfo {
            plate: plate_info.plate.to_owned().unwrap().into(),
            name: plate_info.name().into(),
            plate_type: PlateSetType::from_i32(plate_info.plateType()),
        }
    }
}

#[derive(Debug)]
pub struct Security {
    pub market: QotMarket,
    pub code: String,
}

fn get_qot_market(market: &str) -> QotMarket {
    match market {
        "HK" => QotMarket::QotMarket_HK_Security,
        "US" => QotMarket::QotMarket_US_Security,
        "CNSH" => QotMarket::QotMarket_CNSH_Security,
        "SG" => QotMarket::QotMarket_SG_Security,
        "JP" => QotMarket::QotMarket_JP_Security,
        "CNSZ" => QotMarket::QotMarket_CNSZ_Security,
        _ => QotMarket::QotMarket_Unknown,
    }
}

impl TryFrom<&str> for Security {
    type Error = String;

    fn try_from(code: &str) -> Result<Self, Self::Error> {
        let v: Vec<&str> = code.split('.').collect();
        if v.len() < 2 {
            return Err(format!("Invalid code: {}", code));
        }

        Ok(Security {
            market: get_qot_market(v[0]),
            code: v[1..].join(""),
        })
    }
}

impl ToString for Security {
    fn to_string(&self) -> String {
        let mut market = "UnKnown";
        match self.market {
            QotMarket::QotMarket_HK_Security => market = "HK",
            QotMarket::QotMarket_US_Security => market = "US",
            QotMarket::QotMarket_CNSH_Security => market = "CNSH",
            QotMarket::QotMarket_SG_Security => market = "SG",
            QotMarket::QotMarket_JP_Security => market = "JP",
            QotMarket::QotMarket_CNSZ_Security => market = "CNSZ",
            _ => (),
        }

        format!("{}.{}", market, self.code)
    }
}

pub struct SecurityVec(pub Vec<Security>);

impl Into<Qot_Common::Security> for Security {
    fn into(self) -> Qot_Common::Security {
        let mut security = Qot_Common::Security::new();
        security.set_market(self.market as i32);
        security.set_code(self.code);
        security
    }
}

impl From<Qot_Common::Security> for Security {
    fn from(security: Qot_Common::Security) -> Self {
        Security {
            market: QotMarket::from_i32(security.market()).unwrap(),
            code: security.code().into(),
        }
    }
}

impl Into<Vec<Qot_Common::Security>> for SecurityVec {
    fn into(self) -> Vec<Qot_Common::Security> {
        let mut security_list: Vec<Qot_Common::Security> = Vec::new();
        for security in self.0 {
            security_list.push(security.into())
        }

        security_list
    }
}

#[derive(Debug)]
pub struct PreAfterMarketData {
    price: Option<f64>,
    high_price: Option<f64>,
    low_price: Option<f64>,
    volume: Option<i64>,
    turnover: Option<f64>,
    change_val: Option<f64>,
    change_rate: Option<f64>,
    amplitude: Option<f64>,
}

impl From<Qot_Common::PreAfterMarketData> for PreAfterMarketData {
    fn from(pre_after_market_data: Qot_Common::PreAfterMarketData) -> Self {
        PreAfterMarketData {
            price: pre_after_market_data.price,
            high_price: pre_after_market_data.highPrice,
            low_price: pre_after_market_data.lowPrice,
            volume: pre_after_market_data.volume,
            turnover: pre_after_market_data.turnover,
            change_val: pre_after_market_data.changeVal,
            change_rate: pre_after_market_data.changeVal,
            amplitude: pre_after_market_data.amplitude,
        }
    }
}

#[derive(Debug)]
pub struct ProgramStatus {
    type_: ProgramStatusType,
    str_ext_desc: Option<String>,
}

impl From<Common::ProgramStatus> for ProgramStatus {
    fn from(program_status: Common::ProgramStatus) -> Self {
        ProgramStatus {
            type_: program_status.type_(),
            str_ext_desc: program_status.strExtDesc,
        }
    }
}

#[derive(Debug)]
pub struct BasicQot {
    security: Security,
    is_suspended: bool,
    list_time: String,
    price_spread: f64,
    update_time: String,
    high_price: f64,
    open_price: f64,
    low_price: f64,
    cur_price: f64,
    last_close_price: f64,
    volume: i64,
    turnover: f64,
    turnover_rate: f64,
    amplitude: f64,
    dark_status: Option<DarkStatus>,
    // TODO: option_ex_data: Option<OptionBasicQotExData>,
    list_timestamp: Option<f64>,
    update_timestamp: Option<f64>,
    pre_market: Option<PreAfterMarketData>,
    after_market: Option<PreAfterMarketData>,
    sec_status: Option<SecurityStatus>,
    // TODO: future_ex_data: Option<FutureBasicQotExData>,
}

impl From<Qot_Common::BasicQot> for BasicQot {
    fn from(basic_qot: Qot_Common::BasicQot) -> Self {
        BasicQot {
            security: basic_qot.security.to_owned().unwrap().into(),
            is_suspended: basic_qot.isSuspended(),
            list_time: basic_qot.listTime().into(),
            price_spread: basic_qot.priceSpread(),
            update_time: basic_qot.updateTime().into(),
            high_price: basic_qot.highPrice(),
            open_price: basic_qot.openPrice(),
            low_price: basic_qot.lowPrice(),
            cur_price: basic_qot.curPrice(),
            last_close_price: basic_qot.lastClosePrice(),
            volume: basic_qot.volume(),
            turnover: basic_qot.turnover(),
            turnover_rate: basic_qot.turnoverRate(),
            amplitude: basic_qot.amplitude(),
            dark_status: if basic_qot.darkStatus.is_some() {
                Some(DarkStatus::from_i32(basic_qot.darkStatus()).unwrap())
            } else {
                None
            },
            list_timestamp: basic_qot.listTimestamp,
            update_timestamp: basic_qot.updateTimestamp,
            pre_market: if basic_qot.preMarket.is_some() {
                Some(basic_qot.preMarket.to_owned().unwrap().into())
            } else {
                None
            },
            after_market: if basic_qot.afterMarket.is_some() {
                Some(basic_qot.afterMarket.to_owned().unwrap().into())
            } else {
                None
            },
            sec_status: if basic_qot.secStatus.is_some() {
                Some(SecurityStatus::from_i32(basic_qot.secStatus()).unwrap())
            } else {
                None
            },
        }
    }
}

#[derive(Debug)]
pub struct KLine {
    time: String,
    is_blank: bool,
    close_price: Option<f64>, // 收盘价
}

impl From<Qot_Common::KLine> for KLine {
    fn from(k_line: Qot_Common::KLine) -> Self {
        KLine {
            time: k_line.time().into(),
            is_blank: k_line.isBlank(),
            close_price: k_line.closePrice,
        }
    }
}
