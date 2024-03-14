use crate::{
    Common::RetType,
    Frame,
    KeepAlive::{Request, Response, C2S},
};
use protobuf::MessageField;

pub const PROTO_ID: u32 = 1004;

#[derive(Debug)]
pub struct KeepAliveRequest(i64);

impl Into<Request> for KeepAliveRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_time(self.0);
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl KeepAliveRequest {
    pub fn new(time: i64) -> Self {
        KeepAliveRequest(time)
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct KeepAliveResponse(i64);

impl From<Response> for KeepAliveResponse {
    fn from(resp: Response) -> Self {
        KeepAliveResponse(resp.s2c.time())
    }
}

pub fn check_response(resp: Response) -> crate::Result<KeepAliveResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
