use super::super::common::{BasicQot, Security, SecurityVec};
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_GetBasicQot::{Request, Response, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 3004;

#[derive(Debug)]
pub struct GetBasicQotRequest(Vec<Security>);

impl Into<Request> for GetBasicQotRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.securityList = SecurityVec(self.0).into();
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetBasicQotRequest {
    pub fn new(security_list: Vec<Security>) -> Self {
        GetBasicQotRequest(security_list)
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GetBasicQotResponse(Vec<BasicQot>);

impl From<Response> for GetBasicQotResponse {
    fn from(resp: Response) -> Self {
        let mut basic_qot_list = Vec::new();
        for basic_qot in resp.s2c.basicQotList.iter().cloned() {
            basic_qot_list.push(basic_qot.into());
        }

        GetBasicQotResponse(basic_qot_list)
    }
}

impl GetBasicQotResponse {
    pub fn into_inner(self) -> Vec<BasicQot> {
        self.0
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetBasicQotResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
