use crate::{
    Common::RetType,
    Frame,
    Qot_GetUserSecurityGroup::{GroupType, Request, Response, C2S},
};
use protobuf::{Enum, MessageField};

const PROTO_ID: u32 = 3222;

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
    pub group_name: String,
    pub group_type: GroupType,
}

#[derive(Debug)]
pub struct GetUserSecurityGroupResponse {
    group_list: Vec<GroupData>,
}

impl GetUserSecurityGroupResponse {
    pub fn groups(&self) -> &[GroupData] {
        &self.group_list
    }

    pub fn into_inner(self) -> Vec<GroupData> {
        self.group_list
    }
}

impl From<Response> for GetUserSecurityGroupResponse {
    fn from(resp: Response) -> Self {
        let mut group_list = Vec::new();
        for group_data in &resp.s2c.groupList {
            group_list.push(GroupData {
                group_name: group_data.groupName().into(),
                group_type: GroupType::from_i32(group_data.groupType())
                    .unwrap_or(GroupType::GroupType_Unknown),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Qot_GetUserSecurityGroup::{GroupData as ProtoGroupData, S2C};

    #[test]
    fn builds_request_for_all_groups() {
        let request: Request = GetUserSecurityGroupRequest::new(GroupType::GroupType_All).into();

        assert_eq!(request.c2s.groupType(), GroupType::GroupType_All as i32);
    }

    #[test]
    fn exposes_group_data() {
        let mut custom_group = ProtoGroupData::new();
        custom_group.set_groupName("科技股".into());
        custom_group.set_groupType(GroupType::GroupType_Custom as i32);

        let mut s2c = S2C::new();
        s2c.groupList.push(custom_group);
        let mut response = Response::new();
        response.s2c = MessageField::some(s2c);

        let response: GetUserSecurityGroupResponse = response.into();
        assert_eq!(response.groups().len(), 1);
        assert_eq!(response.groups()[0].group_name, "科技股");
        assert_eq!(response.groups()[0].group_type, GroupType::GroupType_Custom);

        let groups = response.into_inner();
        assert_eq!(groups.len(), 1);
    }

    #[test]
    fn maps_unknown_group_type_without_panicking() {
        let mut unknown_group = ProtoGroupData::new();
        unknown_group.set_groupName("未来分组".into());
        unknown_group.set_groupType(99);

        let mut s2c = S2C::new();
        s2c.groupList.push(unknown_group);
        let mut response = Response::new();
        response.s2c = MessageField::some(s2c);

        let response: GetUserSecurityGroupResponse = response.into();
        assert_eq!(
            response.groups()[0].group_type,
            GroupType::GroupType_Unknown
        );
    }
}
