use crate::{
    action::common::{Security, SecurityVec},
    Common::RetType,
    Frame,
    Qot_ModifyUserSecurity::{ModifyUserSecurityOp, Request, Response, C2S},
};
use protobuf::MessageField;

const PROTO_ID: u32 = 3214;

pub struct ModifyUserSecurityRequest {
    pub group_name: String,
    pub op: ModifyUserSecurityOp,
    pub security_list: Vec<Security>,
}

impl Into<Request> for ModifyUserSecurityRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_groupName(self.group_name);
        c2s.set_op(self.op as i32);
        c2s.securityList = SecurityVec(self.security_list).into();
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl ModifyUserSecurityRequest {
    pub fn new(group_name: String, op: ModifyUserSecurityOp, security_list: Vec<Security>) -> Self {
        ModifyUserSecurityRequest {
            group_name,
            op,
            security_list,
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
