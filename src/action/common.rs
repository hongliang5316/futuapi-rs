use crate::{
    Common::{self, ProgramStatusType},
    Qot_Common::{
        self, DarkStatus, ExchType, PlateSetType, QotMarket, SecurityStatus, SecurityType,
    },
    Trd_Common::{self, OrderStatus, TrdEnv, TrdMarket, TrdSide},
};
use protobuf::Enum;
use serde::Serialize;
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

impl From<Qot_Common::SecurityStaticInfo> for SecurityStaticInfo {
    fn from(security_static_info: Qot_Common::SecurityStaticInfo) -> Self {
        SecurityStaticInfo {
            basic: security_static_info.basic.unwrap().into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct PacketID {
    pub conn_id: u64,
    pub serial_no: u32,
}

impl Into<Common::PacketID> for PacketID {
    fn into(self) -> Common::PacketID {
        let mut packet_id = Common::PacketID::new();
        packet_id.set_connID(self.conn_id);
        packet_id.set_serialNo(self.serial_no);
        packet_id
    }
}

#[derive(Debug, Default, Serialize)]
pub struct TrdFilterConditions {
    pub code_list: Vec<String>,
    pub id_list: Vec<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

impl Into<Trd_Common::TrdFilterConditions> for TrdFilterConditions {
    fn into(self) -> Trd_Common::TrdFilterConditions {
        let mut trd_filter_conditions = Trd_Common::TrdFilterConditions::new();
        trd_filter_conditions.codeList = self.code_list;
        trd_filter_conditions.idList = self.id_list;
        trd_filter_conditions.beginTime = self.begin_time;
        trd_filter_conditions.endTime = self.end_time;
        trd_filter_conditions
    }
}

#[derive(Debug, Default, Clone)]
pub struct Order {
    pub trd_side: TrdSide,
    pub order_id: u64,
    pub code: String,
    pub order_status: OrderStatus,
    pub qty: f64,
    pub price: f64,
}

impl From<Trd_Common::Order> for Order {
    fn from(order: Trd_Common::Order) -> Self {
        Order {
            trd_side: TrdSide::from_i32(order.trdSide()).unwrap(),
            order_id: order.orderID(),
            code: order.code().to_string(),
            order_status: OrderStatus::from_i32(order.orderStatus()).unwrap(),
            qty: order.qty(),
            price: order.price(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Position {
    pub position_id: u64,
    pub code: String,
    pub qty: f64,
    pub can_sell_qty: f64,
    pub price: f64,              // 市价
    pub pl_val: f64,             // 盈亏金额
    pub cost_price: Option<f64>, // 摊薄成本价
    pub pl_ratio: Option<f64>,   // 盈亏百分比
    pub td_pl_val: Option<f64>,  // 今日盈亏金额
}

impl From<Trd_Common::Position> for Position {
    fn from(position: Trd_Common::Position) -> Self {
        Position {
            position_id: position.positionID(),
            code: position.code().to_string(),
            qty: position.qty(),
            can_sell_qty: position.canSellQty(),
            price: position.price(),
            pl_val: position.plVal(),
            cost_price: position.costPrice,
            pl_ratio: position.plRatio,
            td_pl_val: position.td_plVal,
        }
    }
}

#[derive(Debug, Default)]
pub struct TrdHeader {
    pub trd_env: TrdEnv,
    pub acc_id: u64,
    pub trd_market: TrdMarket,
}

impl Into<Trd_Common::TrdHeader> for TrdHeader {
    fn into(self) -> Trd_Common::TrdHeader {
        let mut trd_header = Trd_Common::TrdHeader::new();
        trd_header.set_trdEnv(self.trd_env as i32);
        trd_header.set_accID(self.acc_id);
        trd_header.set_trdMarket(self.trd_market as i32);
        trd_header
    }
}

impl From<Trd_Common::TrdHeader> for TrdHeader {
    fn from(trd_header: Trd_Common::TrdHeader) -> Self {
        TrdHeader {
            trd_env: TrdEnv::from_i32(trd_header.trdEnv()).unwrap(),
            acc_id: trd_header.accID(),
            trd_market: TrdMarket::from_i32(trd_header.trdMarket()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct MaxTrdQtys {
    pub max_cash_buy: f64,
    pub max_cash_and_margin_buy: f64,
    pub max_position_sell: f64,
    pub max_sell_short: Option<f64>,
    pub max_buy_back: Option<f64>,
    pub long_required_i_m: Option<f64>,
    pub short_required_i_m: Option<f64>,
}

impl From<Trd_Common::MaxTrdQtys> for MaxTrdQtys {
    fn from(max_trd_qtys: Trd_Common::MaxTrdQtys) -> Self {
        MaxTrdQtys {
            max_cash_buy: max_trd_qtys.maxCashBuy(),
            max_cash_and_margin_buy: max_trd_qtys.maxCashAndMarginBuy(),
            max_position_sell: max_trd_qtys.maxPositionSell(),
            max_sell_short: max_trd_qtys.maxSellShort,
            max_buy_back: max_trd_qtys.maxBuyBack,
            long_required_i_m: max_trd_qtys.longRequiredIM,
            short_required_i_m: max_trd_qtys.shortRequiredIM,
        }
    }
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

#[derive(Debug, Clone)]
pub struct Security {
    pub market: QotMarket,
    pub code: String,
}

impl PartialEq for Security {
    fn eq(&self, other: &Self) -> bool {
        self.market == other.market && self.code == other.code
    }
}

impl Eq for Security {}

impl std::hash::Hash for Security {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.market.hash(state);
        self.code.hash(state);
    }
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
            code: v[1..].join("."),
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
    pub price: Option<f64>,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    pub volume: Option<i64>,
    pub turnover: Option<f64>,
    pub change_val: Option<f64>,
    pub change_rate: Option<f64>,
    pub amplitude: Option<f64>,
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
            change_rate: pre_after_market_data.changeRate,
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
pub struct KLine {
    pub time: String,
    pub is_blank: bool,
    pub high_price: Option<f64>,
    pub open_price: Option<f64>,
    pub low_price: Option<f64>,
    pub close_price: Option<f64>,
    pub last_close_price: Option<f64>,
    pub volume: Option<i64>,
    pub turnover: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub pe: Option<f64>,
    pub change_rate: Option<f64>,
    pub timestamp: Option<f64>,
}

impl From<Qot_Common::KLine> for KLine {
    fn from(kline: Qot_Common::KLine) -> Self {
        KLine {
            time: kline.time().into(),
            is_blank: kline.isBlank(),
            high_price: kline.highPrice,
            open_price: kline.openPrice,
            low_price: kline.lowPrice,
            close_price: kline.closePrice,
            last_close_price: kline.lastClosePrice,
            volume: kline.volume,
            turnover: kline.turnover,
            turnover_rate: kline.turnoverRate,
            pe: kline.pe,
            change_rate: kline.changeRate,
            timestamp: kline.timestamp,
        }
    }
}

#[derive(Debug)]
pub struct TimeShare {
    pub time: String,
    pub minute: i32,
    pub is_blank: bool,
    pub price: Option<f64>,
    pub last_close_price: Option<f64>,
    pub avg_price: Option<f64>,
    pub volume: Option<i64>,
    pub turnover: Option<f64>,
    pub timestamp: Option<f64>,
}

impl From<Qot_Common::TimeShare> for TimeShare {
    fn from(time_share: Qot_Common::TimeShare) -> Self {
        TimeShare {
            time: time_share.time().into(),
            minute: time_share.minute(),
            is_blank: time_share.isBlank(),
            price: time_share.price,
            last_close_price: time_share.lastClosePrice,
            avg_price: time_share.avgPrice,
            volume: time_share.volume,
            turnover: time_share.turnover,
            timestamp: time_share.timestamp,
        }
    }
}

#[derive(Debug)]
pub struct BasicQot {
    pub security: Security,
    pub is_suspended: bool,
    pub list_time: String,
    pub price_spread: f64,
    pub update_time: String,
    pub high_price: f64,
    pub open_price: f64,
    pub low_price: f64,
    pub cur_price: f64,
    pub last_close_price: f64,
    pub volume: i64,
    pub turnover: f64,
    pub turnover_rate: f64,
    pub amplitude: f64,
    pub dark_status: Option<DarkStatus>,
    // TODO: option_ex_data: Option<OptionBasicQotExData>,
    pub list_timestamp: Option<f64>,
    pub update_timestamp: Option<f64>,
    pub pre_market: Option<PreAfterMarketData>,
    pub after_market: Option<PreAfterMarketData>,
    pub sec_status: Option<SecurityStatus>,
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
