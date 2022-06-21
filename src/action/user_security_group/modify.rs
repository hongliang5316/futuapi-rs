use super::super::common::{Security, SecurityVec};
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_ModifyUserSecurity::{ModifyUserSecurityOp, Request, Response, C2S};
use protobuf::MessageField;

pub const PROTO_ID: u32 = 3214;

#[derive(Debug)]
pub struct ModifyUserSecurityGroupRequest {
    group_name: String,
    op: ModifyUserSecurityOp,
    security_list: Vec<Security>,
}

impl Into<Request> for ModifyUserSecurityGroupRequest {
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

impl ModifyUserSecurityGroupRequest {
    pub fn new(group_name: &str, op: ModifyUserSecurityOp, security_list: Vec<Security>) -> Self {
        ModifyUserSecurityGroupRequest {
            group_name: group_name.into(),
            op,
            security_list,
        }
    }

    pub(crate) fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

pub fn check_response(resp: Response) -> crate::Result<()> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(());
    }

    Err(resp.retMsg().into())
}
