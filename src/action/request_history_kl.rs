use super::common::{KLine, Security};
use crate::{
    Common::RetType,
    Frame,
    Qot_Common::{KLFields, KLType, RehabType},
    Qot_RequestHistoryKL::{Request, Response, C2S},
};
use protobuf::MessageField;

const PROTO_ID: u32 = 3103;

#[derive(Debug)]
pub struct RequestHistoryKLRequest {
    security: Security,
    rehab_type: RehabType,
    kl_type: KLType,
    begin_time: String,
    end_time: String,
    max_ack_kl_num: Option<i32>,
    need_kl_fields_flag: Option<i64>,
    next_req_key: Option<Vec<u8>>,
    extended_time: Option<bool>,
}

impl RequestHistoryKLRequest {
    pub fn new(
        security: Security,
        rehab_type: RehabType,
        kl_type: KLType,
        begin_time: impl Into<String>,
        end_time: impl Into<String>,
    ) -> Self {
        RequestHistoryKLRequest {
            security,
            rehab_type,
            kl_type,
            begin_time: begin_time.into(),
            end_time: end_time.into(),
            max_ack_kl_num: None,
            need_kl_fields_flag: None,
            next_req_key: None,
            extended_time: None,
        }
    }

    pub fn with_max_count(mut self, max_count: i32) -> Self {
        self.max_ack_kl_num = Some(max_count);
        self
    }

    /// Limits the response to the selected K-line fields.
    ///
    /// Passing an empty list clears the field limit and requests all fields.
    pub fn with_fields(mut self, fields: &[KLFields]) -> Self {
        self.need_kl_fields_flag = if fields.is_empty() {
            None
        } else {
            Some(
                fields
                    .iter()
                    .fold(0_i64, |flag, field| flag | (*field as i64)),
            )
        };
        self
    }

    pub fn with_next_req_key(mut self, next_req_key: Vec<u8>) -> Self {
        self.next_req_key = Some(next_req_key);
        self
    }

    pub fn with_extended_time(mut self, extended_time: bool) -> Self {
        self.extended_time = Some(extended_time);
        self
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

impl From<RequestHistoryKLRequest> for Request {
    fn from(request: RequestHistoryKLRequest) -> Self {
        let mut c2s = C2S::new();
        c2s.set_rehabType(request.rehab_type as i32);
        c2s.set_klType(request.kl_type as i32);
        c2s.security = MessageField::some(request.security.into());
        c2s.set_beginTime(request.begin_time);
        c2s.set_endTime(request.end_time);
        c2s.maxAckKLNum = request.max_ack_kl_num;
        c2s.needKLFieldsFlag = request.need_kl_fields_flag;
        c2s.nextReqKey = request.next_req_key;
        c2s.extendedTime = request.extended_time;

        let mut proto_request = Request::new();
        proto_request.c2s = MessageField::some(c2s);
        proto_request
    }
}

#[derive(Debug)]
pub struct RequestHistoryKLResponse {
    pub security: Security,
    pub kl_list: Vec<KLine>,
    pub next_req_key: Option<Vec<u8>>,
}

impl From<Response> for RequestHistoryKLResponse {
    fn from(response: Response) -> Self {
        let mut kl_list = Vec::new();
        for kline in response.s2c.klList.iter().cloned() {
            kl_list.push(kline.into());
        }

        RequestHistoryKLResponse {
            security: response.s2c.security.to_owned().unwrap().into(),
            kl_list,
            next_req_key: response.s2c.nextReqKey.to_owned(),
        }
    }
}

pub fn check_response(response: Response) -> crate::Result<RequestHistoryKLResponse> {
    if response.retType() == RetType::RetType_Succeed as i32 {
        return Ok(response.into());
    }

    Err(format!("{}: {}", response.retType(), response.retMsg()).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Qot_Common::QotMarket;

    #[test]
    fn builds_request_with_optional_fields() {
        let request: Request = RequestHistoryKLRequest::new(
            Security {
                market: QotMarket::QotMarket_US_Security,
                code: "AAPL".into(),
            },
            RehabType::RehabType_Forward,
            KLType::KLType_Week,
            "2025-01-01",
            "2025-12-31",
        )
        .with_max_count(100)
        .with_fields(&[
            KLFields::KLFields_Low,
            KLFields::KLFields_Close,
            KLFields::KLFields_Volume,
            KLFields::KLFields_Turnover,
        ])
        .with_next_req_key(vec![1, 2, 3])
        .with_extended_time(false)
        .into();

        let c2s = request.c2s.as_ref().unwrap();
        assert_eq!(c2s.rehabType(), RehabType::RehabType_Forward as i32);
        assert_eq!(c2s.klType(), KLType::KLType_Week as i32);
        assert_eq!(c2s.security.code(), "AAPL");
        assert_eq!(c2s.beginTime(), "2025-01-01");
        assert_eq!(c2s.endTime(), "2025-12-31");
        assert_eq!(c2s.maxAckKLNum, Some(100));
        assert_eq!(
            c2s.needKLFieldsFlag,
            Some(
                KLFields::KLFields_Low as i64
                    | KLFields::KLFields_Close as i64
                    | KLFields::KLFields_Volume as i64
                    | KLFields::KLFields_Turnover as i64
            )
        );
        assert_eq!(c2s.nextReqKey, Some(vec![1, 2, 3]));
        assert_eq!(c2s.extendedTime, Some(false));
    }

    #[test]
    fn empty_fields_request_all_kline_fields() {
        let request: Request = RequestHistoryKLRequest::new(
            Security {
                market: QotMarket::QotMarket_US_Security,
                code: "AAPL".into(),
            },
            RehabType::RehabType_None,
            KLType::KLType_Day,
            "2025-01-01",
            "2025-01-31",
        )
        .with_fields(&[])
        .into();

        assert_eq!(request.c2s.needKLFieldsFlag, None);
    }

    #[test]
    fn frame_uses_request_history_protocol_id() {
        let frame = RequestHistoryKLRequest::new(
            Security {
                market: QotMarket::QotMarket_US_Security,
                code: "AAPL".into(),
            },
            RehabType::RehabType_None,
            KLType::KLType_Day,
            "2025-01-01",
            "2025-01-31",
        )
        .into_frame();

        assert_eq!(frame.header.proto_id, 3103);
    }
}
