use super::super::common::Security;
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_Common::{PriceReminderFreq, PriceReminderType};
use crate::Qot_SetPriceReminder::{Request, Response, SetPriceReminderOp, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 3220;

pub struct SetPriceReminderRequest {
    security: Security,
    op: SetPriceReminderOp,
    key: Option<i64>,
    type_: Option<PriceReminderType>,
    freq: Option<PriceReminderFreq>,
    value: Option<f64>,
    note: Option<String>,
}

impl Into<Request> for SetPriceReminderRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.security = MessageField::some(self.security.into());
        c2s.set_op(self.op as i32);
        c2s.key = self.key;
        if let Some(type_) = self.type_ {
            c2s.set_type(type_ as i32);
        }

        if let Some(freq) = self.freq {
            c2s.set_freq(freq as i32);
        }

        c2s.value = self.value;
        c2s.note = self.note;

        req.c2s = MessageField::some(c2s);

        req
    }
}

impl SetPriceReminderRequest {
    pub fn new(
        security: Security,
        op: SetPriceReminderOp,
        key: Option<i64>,
        type_: Option<PriceReminderType>,
        freq: Option<PriceReminderFreq>,
        value: Option<f64>,
        note: Option<String>,
    ) -> Self {
        SetPriceReminderRequest {
            security,
            op,
            key,
            type_,
            freq,
            value,
            note,
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct SetPriceReminderResponse(i64);

impl From<Response> for SetPriceReminderResponse {
    fn from(resp: Response) -> Self {
        SetPriceReminderResponse(resp.s2c.key())
    }
}

pub fn check_response(resp: Response) -> crate::Result<SetPriceReminderResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
