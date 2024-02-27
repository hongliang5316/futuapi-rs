use futuapi_rs::{
    action::{common::TrdHeader, max_trd_qtys::GetMaxTrdQtysRequest},
    client, Result,
    Trd_Common::{OrderType, TrdEnv, TrdMarket, TrdSecMarket},
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut trd_client = client::trd_connect("127.0.0.1:11111").await?;
    let get_max_trd_qtys_resp = trd_client
        .get_max_trd_qtys(GetMaxTrdQtysRequest::new(
            TrdHeader {
                trd_env: TrdEnv::TrdEnv_Real,
                acc_id: 0,
                trd_market: TrdMarket::TrdMarket_US,
            },
            OrderType::OrderType_Normal,
            "AAPL".to_string(),
            1.0,
            TrdSecMarket::TrdSecMarket_US,
        ))
        .await?;

    println!("{:?}", get_max_trd_qtys_resp);

    Ok(())
}
