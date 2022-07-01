use futuapi_rs::Trd_Common::{OrderType, TimeInForce, TrdEnv, TrdMarket, TrdSecMarket, TrdSide};
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::trd_connect("127.0.0.1:11111").await?;
    client.unlock("123456".into()).await?;

    let place_order_resp = client
        .place_order(
            123456,
            TrdEnv::TrdEnv_Real,
            TrdMarket::TrdMarket_US,
            TrdSide::TrdSide_Buy,
            OrderType::OrderType_Normal,
            "AAPL".into(),
            100.0,
            Some(1.03),
            None,
            None,
            Some(TrdSecMarket::TrdSecMarket_US),
            Some("bot place order".into()),
            Some(TimeInForce::TimeInForce_DAY),
            Some(true),
            None,
            None,
            None,
            None,
        )
        .await?;

    println!("{:?}", place_order_resp.into_inner());

    Ok(())
}
