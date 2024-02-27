use super::super::common::{PacketID, TrdHeader};
use crate::{
    Common::RetType,
    Frame,
    Trd_Common::{OrderType, TimeInForce, TrailType, TrdSecMarket, TrdSide},
    Trd_PlaceOrder::{Request, Response, C2S},
};
use protobuf::MessageField;

const PROTO_ID: u32 = 2202;

#[derive(Debug, Default)]
pub struct PlaceOrderRequest {
    pub packet_id: PacketID,
    pub header: TrdHeader,
    pub trd_side: TrdSide,
    pub order_type: OrderType,
    pub code: String,
    pub qty: f64,
    pub price: Option<f64>,
    pub adjust_price: Option<bool>,
    pub adjust_side_and_limit: Option<f64>,
    pub sec_market: Option<TrdSecMarket>,
    pub remark: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub fill_outside_rth: Option<bool>,
    pub aux_price: Option<f64>,
    pub trail_type: Option<TrailType>,
    pub trail_value: Option<f64>,
    pub trail_spread: Option<f64>,
}

impl Into<Request> for PlaceOrderRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.packetID = MessageField::some(self.packet_id.into());
        c2s.header = MessageField::some(self.header.into());
        c2s.set_trdSide(self.trd_side as i32);
        c2s.set_orderType(self.order_type as i32);
        c2s.set_code(self.code);
        c2s.set_qty(self.qty);
        c2s.price = self.price;
        c2s.adjustPrice = self.adjust_price;
        c2s.adjustSideAndLimit = self.adjust_side_and_limit;
        if let Some(sec_market) = self.sec_market {
            c2s.set_secMarket(sec_market as i32);
        }
        c2s.remark = self.remark;
        if let Some(time_in_force) = self.time_in_force {
            c2s.set_timeInForce(time_in_force as i32);
        }
        c2s.fillOutsideRTH = self.fill_outside_rth;
        c2s.auxPrice = self.aux_price;
        if let Some(trail_type) = self.trail_type {
            c2s.set_trailType(trail_type as i32);
        }
        c2s.trailValue = self.trail_value;
        c2s.trailSpread = self.trail_spread;
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl PlaceOrderRequest {
    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct PlaceOrderResponse(Option<u64>);

impl From<Response> for PlaceOrderResponse {
    fn from(resp: Response) -> Self {
        PlaceOrderResponse(resp.s2c.orderID)
    }
}

impl PlaceOrderResponse {
    pub fn into_inner(self) -> Option<u64> {
        self.0
    }
}

pub fn check_response(resp: Response) -> crate::Result<PlaceOrderResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
