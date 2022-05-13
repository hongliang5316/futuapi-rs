use super::common::Security;
use crate::frame::Frame;
use crate::Common::RetType;
use crate::Qot_Common::QotMarket;
use crate::Qot_GetIpoList::{Request, Response, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 3217;

#[derive(Debug)]
pub struct GetIpoListRequest(QotMarket);

impl Into<Request> for GetIpoListRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_market(self.into_inner() as i32);
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetIpoListRequest {
    pub fn new(market: QotMarket) -> Self {
        GetIpoListRequest(market)
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }

    pub fn into_inner(self) -> QotMarket {
        self.0
    }
}

#[derive(Debug)]
pub struct BasicIpoData {
    pub security: Security,
    pub name: String,
    pub list_time: Option<String>,
    pub list_timestamp: Option<f64>,
}

#[derive(Debug)]
pub struct IpoData {
    pub basic: BasicIpoData,
}

#[derive(Debug)]
pub struct GetIpoListResponse {
    pub ipo_list: Vec<IpoData>,
}

impl From<Response> for GetIpoListResponse {
    fn from(resp: Response) -> Self {
        let mut ipo_list = Vec::new();
        for ipo_data in &resp.s2c.ipoList {
            ipo_list.push(IpoData {
                basic: BasicIpoData {
                    security: ipo_data.basic.security.to_owned().unwrap().into(),
                    name: ipo_data.basic.name().into(),
                    list_time: ipo_data.basic.listTime.to_owned(),
                    list_timestamp: ipo_data.basic.listTimestamp,
                },
            });
        }

        GetIpoListResponse { ipo_list }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetIpoListResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
