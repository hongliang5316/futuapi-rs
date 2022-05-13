use futuapi_rs::action::ipo::GetIpoListRequest;
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::qot_connect("127.0.0.1:11111").await?;
    let get_ipo_list_resp = client
        .get_ipo_list(GetIpoListRequest::new(
            futuapi_rs::Qot_Common::QotMarket::QotMarket_US_Security,
        ))
        .await?;

    for ipo_data in get_ipo_list_resp.ipo_list {
        println!(
            "{:?}: {:?}",
            ipo_data.basic.list_time,
            ipo_data.basic.security.to_string()
        );
    }

    Ok(())
}
