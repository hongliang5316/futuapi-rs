use super::common::{KLine, Security};
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_Common::{KLType, RehabType};
use crate::Qot_RequestHistoryKL::{Request, Response, C2S, S2C};
use protobuf::MessageField;

const PROTO_ID: u32 = 3103;

#[derive(Debug)]
pub struct RequestHistoryKLRequest {
    rehab_type: RehabType,
    kl_type: KLType,
    security: Security,
    begin_time: String,
    end_time: String,
    max_ack_k_l_num: Option<i32>,
    need_k_l_fields_flag: Option<i64>,
    next_req_key: Option<Vec<u8>>,
    extended_time: Option<bool>,
}

impl Into<Request> for RequestHistoryKLRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_rehabType(self.rehab_type as i32);
        c2s.set_klType(self.kl_type as i32);
        c2s.security = MessageField::some(self.security.into());
        c2s.set_beginTime(self.begin_time);
        c2s.set_endTime(self.end_time);

        c2s.maxAckKLNum = self.max_ack_k_l_num;
        c2s.needKLFieldsFlag = self.need_k_l_fields_flag;
        c2s.nextReqKey = self.next_req_key;
        c2s.extendedTime = self.extended_time;

        req.c2s = MessageField::some(c2s);
        req
    }
}

impl RequestHistoryKLRequest {
    pub fn new(
        rehab_type: RehabType,
        kl_type: KLType,
        security: Security,
        begin_time: String,
        end_time: String,
        max_ack_k_l_num: Option<i32>,
        need_k_l_fields_flag: Option<i64>,
        next_req_key: Option<Vec<u8>>,
        extended_time: Option<bool>,
    ) -> Self {
        RequestHistoryKLRequest {
            rehab_type,
            kl_type,
            security,
            begin_time,
            end_time,
            max_ack_k_l_num,
            need_k_l_fields_flag,
            next_req_key,
            extended_time,
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct RequestHistoryKLResponse {
    security: Security,
    kl_list: Vec<KLine>,
    next_req_key: Option<Vec<u8>>,
}

impl From<Response> for RequestHistoryKLResponse {
    fn from(resp: Response) -> Self {
        // let mut kl_list_new = Vec::new();

        let S2C {
            security,
            // kl_list,
            // next_req_key,
            // special_fields,
        } = resp.s2c.unwrap();
    }
}

pub fn check_response(resp: Response) -> crate::Result<RequestHistoryKLResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
