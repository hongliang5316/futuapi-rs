use super::common::{Security, SecurityStaticInfo};
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_Common::SortField;
use crate::Qot_GetPlateSecurity::{Request, Response, C2S};
use protobuf::MessageField;

pub const PROTO_ID: u32 = 3205;

#[derive(Debug)]
pub struct GetPlateSecurityRequest {
    plate: Security,
    sort_field: Option<SortField>,
    ascend: Option<bool>,
}

impl Into<Request> for GetPlateSecurityRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.plate = MessageField::some(self.plate.into());

        if let Some(sort_field) = self.sort_field {
            c2s.sortField = Some(sort_field as i32);
        }

        c2s.ascend = self.ascend;
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetPlateSecurityRequest {
    pub fn new(plate: Security, sort_field: Option<SortField>, ascend: Option<bool>) -> Self {
        GetPlateSecurityRequest {
            plate,
            sort_field,
            ascend,
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GetPlateSecurityResponse {
    pub static_info_list: Vec<SecurityStaticInfo>,
}

impl From<Response> for GetPlateSecurityResponse {
    fn from(resp: Response) -> Self {
        let mut static_info_list = Vec::new();
        for static_info in &resp.s2c.staticInfoList {
            static_info_list.push(SecurityStaticInfo {
                basic: static_info.basic.to_owned().unwrap().into(),
            });
        }

        GetPlateSecurityResponse { static_info_list }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetPlateSecurityResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
