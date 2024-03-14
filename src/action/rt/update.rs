use super::super::common::{Security, TimeShare};
use crate::{Common::RetType, Qot_UpdateRT::Response};

pub const PROTO_ID: u32 = 3009;

#[derive(Debug)]
pub struct UpdateRTResponse {
    pub security: Security,
    pub rt_list: Vec<TimeShare>,
}

impl From<Response> for UpdateRTResponse {
    fn from(resp: Response) -> Self {
        let mut rt_list = Vec::new();
        for rt in resp.s2c.rtList.iter().cloned() {
            rt_list.push(rt.into());
        }

        UpdateRTResponse {
            security: resp.s2c.security.to_owned().unwrap().into(),
            rt_list,
        }
    }
}

pub fn check_response(resp: Response) -> crate::Result<UpdateRTResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
