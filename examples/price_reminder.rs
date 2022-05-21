use futuapi_rs::action::price_reminder::set::SetPriceReminderRequest;
use futuapi_rs::Qot_SetPriceReminder::SetPriceReminderOp;
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::qot_connect("127.0.0.1:11111").await?;
    let set_price_reminder_resp = client
        .set_price_reminder(SetPriceReminderRequest::new(
            "US.CEI".try_into().unwrap(),
            SetPriceReminderOp::SetPriceReminderOp_DelAll,
            None,
            None,
            None,
            None,
            None,
        ))
        .await?;

    println!("{:?}", set_price_reminder_resp);

    Ok(())
}
