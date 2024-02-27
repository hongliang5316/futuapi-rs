use super::super::common::SecurityStaticInfo;
use crate::{
    Common::RetType,
    Frame,
    Qot_GetUserSecurity::{Request, Response, C2S},
};
use protobuf::MessageField;

const PROTO_ID: u32 = 3213;

#[derive(Debug)]
pub struct GetUserSecurityRequest(String);

impl Into<Request> for GetUserSecurityRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_groupName(self.0);
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetUserSecurityRequest {
    pub fn new(group_name: String) -> Self {
        GetUserSecurityRequest(group_name)
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GetUserSecurityResponse(Vec<SecurityStaticInfo>);

impl GetUserSecurityResponse {
    pub fn into_inner(self) -> Vec<SecurityStaticInfo> {
        self.0
    }
}

impl From<Response> for GetUserSecurityResponse {
    fn from(resp: Response) -> Self {
        let mut security_static_info_list = Vec::new();
        for security_static_info in resp.s2c.staticInfoList.iter().cloned() {
            security_static_info_list.push(security_static_info.into());
        }

        GetUserSecurityResponse(security_static_info_list)
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetUserSecurityResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
