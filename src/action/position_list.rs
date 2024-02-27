use super::common::{Position, TrdHeader};
use crate::{
    Common::RetType,
    Frame,
    Trd_GetPositionList::{Request, Response, C2S},
};
use protobuf::MessageField;

const PROTO_ID: u32 = 2102;

#[derive(Debug, Default)]
pub struct GetPositionListRequest {
    pub header: TrdHeader,
    pub refresh_cache: Option<bool>,
}

impl Into<Request> for GetPositionListRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.header = MessageField::some(self.header.into());
        c2s.refreshCache = self.refresh_cache;
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetPositionListRequest {
    pub fn new(header: TrdHeader, refresh_cache: Option<bool>) -> Self {
        GetPositionListRequest {
            header,
            refresh_cache,
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GetPositionListResponse {
    // header: TrdHeader,
    pub position_list: Vec<Position>,
}

impl From<Response> for GetPositionListResponse {
    fn from(resp: Response) -> Self {
        let mut position_list = Vec::new();
        for position in resp.s2c.unwrap().positionList {
            position_list.push(position.into());
        }

        GetPositionListResponse { position_list }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetPositionListResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
