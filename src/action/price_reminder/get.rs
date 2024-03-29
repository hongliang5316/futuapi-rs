use super::super::common::Security;
use crate::{
    Common::RetType,
    Frame,
    Qot_Common::{PriceReminderFreq, PriceReminderType, QotMarket},
    Qot_GetPriceReminder::{self, Request, Response, C2S},
};
use protobuf::{Enum, MessageField};

const PROTO_ID: u32 = 3221;

pub struct GetPriceReminderRequest {
    pub security: Option<Security>,
    pub market: Option<QotMarket>,
}

impl Into<Request> for GetPriceReminderRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();

        if let Some(security) = self.security {
            c2s.security = MessageField::some(security.into());
        }

        if let Some(market) = self.market {
            c2s.set_market(market as i32);
        }

        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetPriceReminderRequest {
    pub fn new(security: Option<Security>, market: Option<QotMarket>) -> Self {
        GetPriceReminderRequest { security, market }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct PriceReminderItem {
    pub key: i64,
    pub type_: PriceReminderType,
    pub value: f64,
    pub note: String,
    pub freq: PriceReminderFreq,
    pub is_enable: bool,
}

impl From<Qot_GetPriceReminder::PriceReminderItem> for PriceReminderItem {
    fn from(item: Qot_GetPriceReminder::PriceReminderItem) -> Self {
        PriceReminderItem {
            key: item.key(),
            type_: PriceReminderType::from_i32(item.type_()).unwrap(),
            value: item.value(),
            note: item.note().into(),
            freq: PriceReminderFreq::from_i32(item.freq()).unwrap(),
            is_enable: item.isEnable(),
        }
    }
}

#[derive(Debug)]
pub struct PriceReminder {
    pub security: Security,
    pub item_list: Vec<PriceReminderItem>,
}

impl From<Qot_GetPriceReminder::PriceReminder> for PriceReminder {
    fn from(price_reminder: Qot_GetPriceReminder::PriceReminder) -> Self {
        let mut item_list = Vec::new();
        for item in price_reminder.itemList {
            item_list.push(item.into());
        }

        PriceReminder {
            security: price_reminder.security.unwrap().into(),
            item_list,
        }
    }
}

#[derive(Debug)]
pub struct GetPriceReminderResponse(pub Vec<PriceReminder>);

impl From<Response> for GetPriceReminderResponse {
    fn from(resp: Response) -> Self {
        let mut price_reminder_list = Vec::new();
        for price_reminder in resp.s2c.priceReminderList.iter().cloned() {
            price_reminder_list.push(price_reminder.into());
        }

        GetPriceReminderResponse(price_reminder_list)
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetPriceReminderResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
