use futuapi_rs::{
    action::{common::Security, security_snapshot::GetSecuritySnapshotRequest},
    client,
    Qot_Common::QotMarket,
    Result,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut qot_client = client::qot_connect("127.0.0.1:11111").await?;

    let get_security_snapshot_resp = qot_client
        .get_security_snapshot(GetSecuritySnapshotRequest::new(vec![Security {
            market: QotMarket::QotMarket_US_Security,
            code: "AAPL".into(),
        }]))
        .await?;

    println!("{:?}", get_security_snapshot_resp);
    for snapshot in get_security_snapshot_resp.snapshot_list {
        if let Some(equity) = snapshot.equity_ex_data {
            println!(
                "last price: {}, issued shares: {}, outstanding shares: {}",
                snapshot.basic.cur_price, equity.issued_shares, equity.outstanding_shares
            );
        }
    }

    Ok(())
}
