use crate::Common::RetType;
use crate::Frame;
use crate::Trd_Common::SecurityFirm;
use crate::Trd_UnlockTrade::{Request, Response, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 2005;

#[derive(Debug)]
pub struct UnlockRequest {
    unlock: bool,
    pwd_md5: Option<String>,
    security_firm: Option<SecurityFirm>,
}

impl Into<Request> for UnlockRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_unlock(self.unlock);
        c2s.pwdMD5 = self.pwd_md5;
        if let Some(security_firm) = self.security_firm {
            c2s.set_securityFirm(security_firm as i32);
        }

        req.c2s = MessageField::some(c2s);

        req
    }
}

impl UnlockRequest {
    pub fn new(pwd_md5: String, security_firm: Option<SecurityFirm>) -> Self {
        UnlockRequest {
            unlock: true,
            pwd_md5: Some(pwd_md5),
            security_firm,
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

pub fn check_response(resp: Response) -> crate::Result<()> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(());
    }

    Err(format!("{}: {}", resp.retType(), resp.retMsg()).into())
}
