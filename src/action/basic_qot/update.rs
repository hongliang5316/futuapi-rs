use super::super::common::BasicQot;
use crate::Common::RetType;
use crate::Qot_UpdateBasicQot::Response;

pub const PROTO_ID: u32 = 3005;

#[derive(Debug)]
pub struct UpdateBasicQotResponse(pub Vec<BasicQot>);

impl From<Response> for UpdateBasicQotResponse {
    fn from(resp: Response) -> Self {
        let mut basic_qot_list = Vec::new();
        for basic_qot in resp.s2c.basicQotList.iter().cloned() {
            basic_qot_list.push(basic_qot.into());
        }

        UpdateBasicQotResponse(basic_qot_list)
    }
}

pub fn check_response(resp: Response) -> crate::Result<UpdateBasicQotResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
