use crate::action::common::ProgramStatus;
use crate::Common::RetType;
use crate::Frame;
use crate::GetGlobalState::{Request, Response, C2S};
use crate::Qot_Common::QotMarketState;
use protobuf::{Enum, MessageField};

pub const PROTO_ID: u32 = 1002;

pub struct GetGlobalStateRequest;

impl Into<Request> for GetGlobalStateRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_userID(0);
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetGlobalStateRequest {
    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GetGlobalStateResponse {
    pub market_hk: QotMarketState,
    pub market_us: QotMarketState,
    pub market_sh: QotMarketState,
    pub market_sz: QotMarketState,
    pub market_hk_future: QotMarketState,
    pub market_us_future: Option<QotMarketState>,
    pub market_sg_future: Option<QotMarketState>,
    pub market_jp_future: Option<QotMarketState>,
    pub qot_logined: bool,
    pub trd_logined: bool,
    pub server_ver: i32,
    pub server_build_no: i32,
    pub time: i64,
    pub local_time: Option<f64>,
    pub program_status: Option<ProgramStatus>,
    pub qot_svr_ip_addr: Option<String>,
    pub trd_svr_ip_addr: Option<String>,
    pub conn_id: Option<u64>,
}

impl From<Response> for GetGlobalStateResponse {
    fn from(resp: Response) -> Self {
        GetGlobalStateResponse {
            market_hk: QotMarketState::from_i32(resp.s2c.marketHK()).unwrap(),
            market_us: QotMarketState::from_i32(resp.s2c.marketUS()).unwrap(),
            market_sh: QotMarketState::from_i32(resp.s2c.marketSH()).unwrap(),
            market_sz: QotMarketState::from_i32(resp.s2c.marketSZ()).unwrap(),
            market_hk_future: QotMarketState::from_i32(resp.s2c.marketHKFuture()).unwrap(),
            market_us_future: if resp.s2c.marketUSFuture.is_some() {
                QotMarketState::from_i32(resp.s2c.marketUSFuture())
            } else {
                None
            },
            market_sg_future: if resp.s2c.marketSGFuture.is_some() {
                QotMarketState::from_i32(resp.s2c.marketSGFuture())
            } else {
                None
            },
            market_jp_future: if resp.s2c.marketJPFuture.is_some() {
                QotMarketState::from_i32(resp.s2c.marketJPFuture())
            } else {
                None
            },
            qot_logined: resp.s2c.qotLogined(),
            trd_logined: resp.s2c.trdLogined(),
            server_ver: resp.s2c.serverVer(),
            server_build_no: resp.s2c.serverBuildNo(),
            time: resp.s2c.time(),
            local_time: resp.s2c.localTime,
            program_status: if resp.s2c.programStatus.is_some() {
                Some(resp.s2c.programStatus.to_owned().unwrap().into())
            } else {
                None
            },
            qot_svr_ip_addr: resp.s2c.qotSvrIpAddr.to_owned(),
            trd_svr_ip_addr: resp.s2c.trdSvrIpAddr.to_owned(),
            conn_id: resp.s2c.connID,
        }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetGlobalStateResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
