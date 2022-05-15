use super::common::{Security, SecurityVec};
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_Common::{RehabType, SubType};
use crate::Qot_Sub::{Request, Response, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 3001;

#[derive(Debug)]
pub struct SubscribeRequest {
    security_list: Vec<Security>,
    sub_type_list: Vec<SubType>,
    is_sub_or_un_sub: bool,
    is_reg_or_un_reg_push: Option<bool>,
    reg_push_rehab_type_list: Vec<RehabType>,
    is_first_push: Option<bool>,
    is_unsub_all: Option<bool>,
    is_sub_order_book_detail: Option<bool>,
    extended_time: Option<bool>,
}

impl Into<Request> for SubscribeRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.securityList = SecurityVec(self.security_list).into();
        c2s.subTypeList = self
            .sub_type_list
            .into_iter()
            .map(|sub_type| sub_type as i32)
            .collect();
        c2s.set_isSubOrUnSub(self.is_sub_or_un_sub);
        if let Some(is_reg_or_un_reg_push) = self.is_reg_or_un_reg_push {
            c2s.set_isRegOrUnRegPush(is_reg_or_un_reg_push);
        }

        c2s.regPushRehabTypeList = self
            .reg_push_rehab_type_list
            .into_iter()
            .map(|rehab_type| rehab_type as i32)
            .collect();

        if let Some(is_first_push) = self.is_first_push {
            c2s.set_isFirstPush(is_first_push);
        }

        if let Some(is_unsub_all) = self.is_unsub_all {
            c2s.set_isUnsubAll(is_unsub_all);
        }

        if let Some(is_sub_order_book_detail) = self.is_sub_order_book_detail {
            c2s.set_isSubOrderBookDetail(is_sub_order_book_detail);
        }

        if let Some(extended_time) = self.extended_time {
            c2s.set_extendedTime(extended_time);
        }

        req.c2s = MessageField::some(c2s);
        req
    }
}

impl SubscribeRequest {
    pub fn new(
        security_list: Vec<Security>,
        sub_type_list: Vec<SubType>,
        is_sub_or_un_sub: bool,
        is_reg_or_un_reg_push: Option<bool>,
        reg_push_rehab_type_list: Vec<RehabType>,
        is_first_push: Option<bool>,
        is_unsub_all: Option<bool>,
        is_sub_order_book_detail: Option<bool>,
        extended_time: Option<bool>,
    ) -> Self {
        SubscribeRequest {
            security_list,
            sub_type_list,
            is_sub_or_un_sub,
            is_reg_or_un_reg_push,
            reg_push_rehab_type_list,
            is_first_push,
            is_unsub_all,
            is_sub_order_book_detail,
            extended_time,
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

    Err(format!("{}: {}", resp.retType(), resp.retMsg()).into())
}
