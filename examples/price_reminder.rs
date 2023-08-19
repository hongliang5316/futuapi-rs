use futuapi_rs::action::price_reminder::{
    get::GetPriceReminderRequest, set::SetPriceReminderRequest,
};
use futuapi_rs::Qot_Common::{PriceReminderFreq, PriceReminderType};
use futuapi_rs::Qot_SetPriceReminder::SetPriceReminderOp;
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut qot_client = client::qot_connect("127.0.0.1:11111").await?;
    let set_price_reminder_resp = qot_client
        .set_price_reminder(SetPriceReminderRequest::new(
            "US.CEI".try_into().unwrap(),
            SetPriceReminderOp::SetPriceReminderOp_Add,
            None,
            Some(PriceReminderType::PriceReminderType_PriceUp),
            Some(PriceReminderFreq::PriceReminderFreq_OnlyOnce),
            Some(1.1),
            Some("test".into()),
        ))
        .await?;

    println!("{:?}", set_price_reminder_resp);

    let get_price_reminder_resp = qot_client
        .get_price_reminder(GetPriceReminderRequest::new(
            Some("US.CEI".try_into().unwrap()),
            None,
        ))
        .await?;

    println!("{:?}", get_price_reminder_resp);

    qot_client
        .set_price_reminder(SetPriceReminderRequest::new(
            "US.CEI".try_into().unwrap(),
            SetPriceReminderOp::SetPriceReminderOp_DelAll,
            None,
            None,
            None,
            None,
            Some("test".into()),
        ))
        .await?;

    Ok(())
}
