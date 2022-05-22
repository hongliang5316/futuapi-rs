use crate::Common::RetType;
use crate::Frame;
use crate::Qot_GetUserSecurityGroup::{GroupType, Request, Response, C2S};
use protobuf::{Enum, MessageField};

pub const PROTO_ID: u32 = 3222;

#[derive(Debug)]
pub struct GetUserSecurityGroupRequest(GroupType);

impl Into<Request> for GetUserSecurityGroupRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_groupType(self.0 as i32);
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetUserSecurityGroupRequest {
    pub fn new(group_type: GroupType) -> Self {
        GetUserSecurityGroupRequest(group_type)
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GroupData {
    group_name: String,
    group_type: GroupType,
}

#[derive(Debug)]
pub struct GetUserSecurityGroupResponse {
    group_list: Vec<GroupData>,
}

impl From<Response> for GetUserSecurityGroupResponse {
    fn from(resp: Response) -> Self {
        let mut group_list = Vec::new();
        for group_data in &resp.s2c.groupList {
            group_list.push(GroupData {
                group_name: group_data.groupName().into(),
                group_type: GroupType::from_i32(group_data.groupType()).unwrap(),
            })
        }

        GetUserSecurityGroupResponse { group_list }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetUserSecurityGroupResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
