use futuapi_rs::action::ipo::GetIpoListRequest;
use futuapi_rs::action::subscribe::SubscribeRequest;
use futuapi_rs::Qot_Common::SubType;
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::qot_connect("127.0.0.1:11111").await?;

    client
        .subscribe(SubscribeRequest::new(
            vec![
                "HK.00700".try_into().unwrap(),
                "HK.01024".try_into().unwrap(),
            ],
            vec![SubType::SubType_Basic],
            true,
            Some(true),
            Vec::new(),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
        ))
        .await?;

    client
        .get_ipo_list(GetIpoListRequest::new(
            futuapi_rs::Qot_Common::QotMarket::QotMarket_US_Security,
        ))
        .await?;

    Ok(())
}
