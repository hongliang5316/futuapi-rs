use super::common::{Order, TrdFilterConditions, TrdHeader};
use crate::{
    Common::RetType,
    Frame,
    Trd_Common::OrderStatus,
    Trd_GetHistoryOrderList::{Request, Response, C2S},
};
use chrono::Duration;
use protobuf::MessageField;

const PROTO_ID: u32 = 2221;

pub struct GetHistoryOrderListRequest {
    pub header: TrdHeader,
    pub filter_conditions: TrdFilterConditions,
    pub filter_status_list: Vec<OrderStatus>,
}

impl GetHistoryOrderListRequest {
    pub fn new(
        header: TrdHeader,
        mut filter_conditions: TrdFilterConditions,
        filter_status_list: Vec<OrderStatus>,
    ) -> Self {
        if filter_conditions.begin_time.is_none() && filter_conditions.end_time.is_none() {
            let now = chrono::offset::Local::now();
            filter_conditions.begin_time =
                Some((now - Duration::days(89)).format("%Y-%m-%d").to_string());
            filter_conditions.end_time =
                Some((now + Duration::days(1)).format("%Y-%m-%d").to_string());
        }

        GetHistoryOrderListRequest {
            header,
            filter_conditions,
            filter_status_list,
        }
    }
}

impl Into<Request> for GetHistoryOrderListRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();

        c2s.header = MessageField::some(self.header.into());
        c2s.filterConditions = MessageField::some(self.filter_conditions.into());
        c2s.filterStatusList = self
            .filter_status_list
            .into_iter()
            .map(|x| x as i32)
            .collect();
        req.c2s = MessageField::some(c2s);
        req
    }
}

impl GetHistoryOrderListRequest {
    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct GetHistoryOrderListResponse {
    pub order_list: Vec<Order>,
}

impl From<Response> for GetHistoryOrderListResponse {
    fn from(resp: Response) -> Self {
        let mut order_list = Vec::new();
        for order in resp.s2c.orderList.iter().cloned() {
            order_list.push(order.into());
        }

        GetHistoryOrderListResponse { order_list }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetHistoryOrderListResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
