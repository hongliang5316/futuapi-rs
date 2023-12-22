use super::super::common::{PacketID, TrdHeader};
use crate::Common::RetType;
use crate::Frame;
use crate::Trd_Common::ModifyOrderOp;
use crate::Trd_ModifyOrder::{Request, Response, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 2205;

#[derive(Debug, Default)]
pub struct ModifyOrderRequest {
    pub packet_id: PacketID,
    pub header: TrdHeader,
    pub order_id: u64,
    pub modify_order_op: ModifyOrderOp,
    pub qty: Option<f64>,
    pub price: Option<f64>,
}

impl Into<Request> for ModifyOrderRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.packetID = MessageField::some(self.packet_id.into());
        c2s.header = MessageField::some(self.header.into());
        c2s.set_orderID(self.order_id);
        c2s.set_modifyOrderOp(self.modify_order_op as i32);
        c2s.qty = self.qty;
        c2s.price = self.price;

        req.c2s = MessageField::some(c2s);
        req
    }
}

impl ModifyOrderRequest {
    pub fn new(
        packet_id: PacketID,
        header: TrdHeader,
        order_id: u64,
        modify_order_op: ModifyOrderOp,
        qty: Option<f64>,
        price: Option<f64>,
    ) -> Self {
        ModifyOrderRequest {
            packet_id,
            header,
            order_id,
            modify_order_op,
            qty,
            price,
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct ModifyOrderResponse {
    pub order_id: u64,
}

impl From<Response> for ModifyOrderResponse {
    fn from(resp: Response) -> Self {
        ModifyOrderResponse {
            order_id: resp.s2c.orderID(),
        }
    }
}

pub fn check_response(resp: Response) -> crate::Result<ModifyOrderResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
