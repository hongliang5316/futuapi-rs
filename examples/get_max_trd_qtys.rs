use futuapi_rs::action::common::TrdHeader;
use futuapi_rs::action::max_trd_qtys::GetMaxTrdQtysRequest;
use futuapi_rs::Trd_Common::{OrderType, TrdEnv, TrdMarket, TrdSecMarket};
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::trd_connect("127.0.0.1:11111").await?;
    let get_max_trd_qtys_resp = client
        .get_max_trd_qtys(GetMaxTrdQtysRequest::new(
            TrdHeader {
                trd_env: TrdEnv::TrdEnv_Real,
                acc_id: 0,
                trd_market: TrdMarket::TrdMarket_US,
            },
            OrderType::OrderType_Normal,
            "AAPL".to_string(),
            1.0,
        ))
        .await?;

    println!("{:?}", get_max_trd_qtys_resp);

    Ok(())
}
