use futuapi_rs::{
    action::{common::Security, request_history_kl::RequestHistoryKLRequest},
    client,
    Qot_Common::{KLFields, KLType, QotMarket, RehabType},
    Result,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut qot_client = client::qot_connect("127.0.0.1:11111").await?;

    let response = qot_client
        .request_history_kl(
            RequestHistoryKLRequest::new(
                Security {
                    market: QotMarket::QotMarket_US_Security,
                    code: "AAPL".into(),
                },
                RehabType::RehabType_None,
                KLType::KLType_Week,
                "2025-01-01",
                "2025-12-31",
            )
            .with_max_count(100)
            .with_fields(&[
                KLFields::KLFields_Low,
                KLFields::KLFields_Close,
                KLFields::KLFields_Volume,
                KLFields::KLFields_Turnover,
            ]),
        )
        .await?;

    println!("{:?}", response);

    Ok(())
}
