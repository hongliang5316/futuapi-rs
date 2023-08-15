use super::common::{MaxTrdQtys, TrdHeader};
use crate::Common::RetType;
use crate::Frame;
use crate::Trd_Common::{OrderType, TrdSecMarket};
use crate::Trd_GetMaxTrdQtys::{Request, Response, C2S, S2C};
use protobuf::MessageField;

const PROTO_ID: u32 = 2111;

pub struct GetMaxTrdQtysRequest {
    header: TrdHeader,
    order_type: OrderType,
    code: String,
    price: f64,
    order_id: Option<u64>,
    adjust_price: Option<bool>,
    adjust_side_and_limit: Option<f64>,
    sec_market: Option<TrdSecMarket>,
}

impl Into<Request> for GetMaxTrdQtysRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();

        c2s.header = MessageField::some(self.header.into());
        c2s.set_orderType(self.order_type as i32);
        c2s.set_code(self.code);
        c2s.set_price(self.price);
        c2s.orderID = self.order_id;
        c2s.adjustPrice = self.adjust_price;
        c2s.adjustSideAndLimit = self.adjust_side_and_limit;
        if let Some(sec_market) = self.sec_market {
            c2s.set_secMarket(sec_market as i32);
        }

        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetMaxTrdQtysRequest {
    pub fn new(
        header: TrdHeader,
        order_type: OrderType,
        code: String,
        price: f64,
        sec_market: TrdSecMarket,
    ) -> Self {
        GetMaxTrdQtysRequest {
            header,
            order_type,
            code,
            price,
            order_id: None,
            adjust_price: None,
            adjust_side_and_limit: None,
            sec_market: Some(sec_market),
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GetMaxTrdQtysResponse {
    pub header: TrdHeader,
    pub max_trd_qtys: Option<MaxTrdQtys>,
}

impl From<Response> for GetMaxTrdQtysResponse {
    fn from(resp: Response) -> Self {
        let S2C {
            header,
            maxTrdQtys: max_trd_qtys,
            ..
        } = resp.s2c.unwrap();

        let mut get_max_trd_qtys_resp = GetMaxTrdQtysResponse {
            header: header.unwrap().into(),
            max_trd_qtys: None,
        };

        if max_trd_qtys.is_some() {
            get_max_trd_qtys_resp.max_trd_qtys = Some(max_trd_qtys.unwrap().into())
        }

        get_max_trd_qtys_resp
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetMaxTrdQtysResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
