use super::common::{PreAfterMarketData, Security, SecurityVec};
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_GetSecuritySnapshot::{self, Request, Response, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 3203;

#[derive(Debug)]
pub struct GetSecuritySnapshotRequest(Vec<Security>);

impl Into<Request> for GetSecuritySnapshotRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.securityList = SecurityVec(self.0).into();
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetSecuritySnapshotRequest {
    pub fn new(security_list: Vec<Security>) -> Self {
        GetSecuritySnapshotRequest(security_list)
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct SnapshotBasicData {
    pub security: Security,
    pub type_: i32,
    pub is_suspend: bool,
    pub list_time: String,
    pub log_size: i32,
    pub price_spread: f64,
    pub update_time: String,
    pub high_price: f64,
    pub open_price: f64,
    pub low_price: f64,
    pub last_close_price: f64,
    pub cur_price: f64,
    pub volume: i64,
    pub turnover: f64,      // 成交额
    pub turnover_rate: f64, // 换手率
    pub list_timestamp: Option<f64>,
    pub update_timestamp: Option<f64>,
    pub ask_price: Option<f64>,
    pub bid_price: Option<f64>,
    pub ask_vol: Option<i64>,
    pub bid_vol: Option<i64>,
    pub enable_margin: Option<bool>,
    pub mortgage_ratio: Option<f64>,
    pub long_margin_initial_ratio: Option<f64>,
    pub enable_short_sell: Option<bool>,
    pub short_sell_rate: Option<f64>,
    pub short_available_volume: Option<i64>,
    pub short_margin_initial_ratio: Option<f64>,
    pub amplitude: Option<f64>,
    pub avg_price: Option<f64>,
    pub bid_ask_ratio: Option<f64>,
    pub volume_ratio: Option<f64>,
    pub highest_52_weeks_price: Option<f64>,
    pub lowest_52_weeks_price: Option<f64>,
    pub highest_history_price: Option<f64>,
    pub lowest_history_price: Option<f64>,
    pub pre_market: Option<PreAfterMarketData>,
    pub after_market: Option<PreAfterMarketData>,
    pub sec_status: Option<i32>,
    pub close_price_5_minute: Option<f64>,
}

impl From<Qot_GetSecuritySnapshot::SnapshotBasicData> for SnapshotBasicData {
    fn from(snapshot_basic_data: Qot_GetSecuritySnapshot::SnapshotBasicData) -> Self {
        SnapshotBasicData {
            security: snapshot_basic_data.security.to_owned().unwrap().into(),
            type_: snapshot_basic_data.type_(),
            is_suspend: snapshot_basic_data.isSuspend(),
            list_time: snapshot_basic_data.listTime().into(),
            log_size: snapshot_basic_data.lotSize(),
            price_spread: snapshot_basic_data.priceSpread(),
            update_time: snapshot_basic_data.updateTime().into(),
            high_price: snapshot_basic_data.highPrice(),
            open_price: snapshot_basic_data.openPrice(),
            low_price: snapshot_basic_data.lowPrice(),
            last_close_price: snapshot_basic_data.lastClosePrice(),
            cur_price: snapshot_basic_data.curPrice(),
            volume: snapshot_basic_data.volume(),
            turnover: snapshot_basic_data.turnover(),
            turnover_rate: snapshot_basic_data.turnoverRate(),
            list_timestamp: snapshot_basic_data.listTimestamp,
            update_timestamp: snapshot_basic_data.updateTimestamp,
            ask_price: snapshot_basic_data.askPrice,
            bid_price: snapshot_basic_data.bidPrice,
            ask_vol: snapshot_basic_data.askVol,
            bid_vol: snapshot_basic_data.bidVol,
            enable_margin: snapshot_basic_data.enableMargin,
            mortgage_ratio: snapshot_basic_data.mortgageRatio,
            long_margin_initial_ratio: snapshot_basic_data.longMarginInitialRatio,
            enable_short_sell: snapshot_basic_data.enableShortSell,
            short_sell_rate: snapshot_basic_data.shortSellRate,
            short_available_volume: snapshot_basic_data.shortAvailableVolume,
            short_margin_initial_ratio: snapshot_basic_data.shortMarginInitialRatio,
            amplitude: snapshot_basic_data.amplitude,
            avg_price: snapshot_basic_data.avgPrice,
            bid_ask_ratio: snapshot_basic_data.bidAskRatio,
            volume_ratio: snapshot_basic_data.volumeRatio,
            highest_52_weeks_price: snapshot_basic_data.highest52WeeksPrice,
            lowest_52_weeks_price: snapshot_basic_data.lowest52WeeksPrice,
            highest_history_price: snapshot_basic_data.highestHistoryPrice,
            lowest_history_price: snapshot_basic_data.lowestHistoryPrice,
            pre_market: if snapshot_basic_data.preMarket.is_some() {
                Some(snapshot_basic_data.preMarket.unwrap().into())
            } else {
                None
            },
            after_market: if snapshot_basic_data.afterMarket.is_some() {
                Some(snapshot_basic_data.afterMarket.unwrap().into())
            } else {
                None
            },
            sec_status: snapshot_basic_data.secStatus,
            close_price_5_minute: snapshot_basic_data.closePrice5Minute,
        }
    }
}

#[derive(Debug)]
pub struct Snapshot {
    pub basic: SnapshotBasicData,
}

#[derive(Debug)]
pub struct GetSecuritySnapshotResponse {
    pub snapshot_list: Vec<Snapshot>,
}

impl From<Response> for GetSecuritySnapshotResponse {
    fn from(resp: Response) -> Self {
        let mut snapshot_list = Vec::new();
        for snapshot in &resp.s2c.snapshotList {
            snapshot_list.push(Snapshot {
                basic: snapshot.basic.to_owned().unwrap().into(),
            });
        }

        GetSecuritySnapshotResponse { snapshot_list }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetSecuritySnapshotResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
