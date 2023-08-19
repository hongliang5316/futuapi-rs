use futuapi_rs::action::common::Security;
use futuapi_rs::action::security_snapshot::GetSecuritySnapshotRequest;
use futuapi_rs::Qot_Common::QotMarket;
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut qot_client = client::qot_connect("127.0.0.1:11111").await?;

    let get_security_snapshot_resp = qot_client
        .get_security_snapshot(GetSecuritySnapshotRequest::new(vec![Security {
            market: QotMarket::QotMarket_US_Security,
            code: "CEI".into(),
        }]))
        .await?;

    println!("{:?}", get_security_snapshot_resp);

    Ok(())
}
