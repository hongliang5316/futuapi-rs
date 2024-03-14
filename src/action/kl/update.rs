use super::super::common::{KLine, Security};
use crate::{Common::RetType, Qot_Common::KLType, Qot_UpdateKL::Response};
use protobuf::Enum;

pub const PROTO_ID: u32 = 3007;

#[derive(Debug)]
pub struct UpdateKLResponse {
    pub rehab_type: i32,
    pub kl_type: KLType,
    pub security: Security,
    pub kl_list: Vec<KLine>,
}

impl From<Response> for UpdateKLResponse {
    fn from(resp: Response) -> Self {
        let mut kl_list = Vec::new();
        for kl in resp.s2c.klList.iter().cloned() {
            kl_list.push(kl.into());
        }

        UpdateKLResponse {
            rehab_type: resp.s2c.rehabType(),
            kl_type: KLType::from_i32(resp.s2c.klType()).unwrap(),
            security: resp.s2c.security.to_owned().unwrap().into(),
            kl_list,
        }
    }
}

pub fn check_response(resp: Response) -> crate::Result<UpdateKLResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
