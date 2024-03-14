use futuapi_rs::{
    action::subscribe::SubscribeRequest, client, frame::Error, Qot_Common::SubType, Result,
    UpdateResponse,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let sub_client = client::sub_connect("127.0.0.1:11111").await?;
    let mut sub = sub_client
        .subscribe(SubscribeRequest::new(
            vec!["HK.00700".try_into().unwrap()],
            vec![
                SubType::SubType_Basic,
                SubType::SubType_RT,
                SubType::SubType_KL_1Min,
            ],
            true,
            Some(true),
            Vec::new(),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
        ))
        .await?;

    loop {
        match sub.next_data().await {
            Ok(Some(update_resp)) => match update_resp {
                UpdateResponse::BasicQot(update_basic_qot_resp) => {
                    println!("{:?}", update_basic_qot_resp);
                }
                UpdateResponse::RT(update_rt_resp) => {
                    println!("{:?}", update_rt_resp);
                }
                UpdateResponse::KL(update_kl_resp) => {
                    println!("{:?}", update_kl_resp);
                }
            },
            Ok(None) => {
                continue;
            }
            Err(e) => {
                let err = e.downcast::<Error>()?;
                if let Error::Timeout(_) = *err {
                    continue;
                }

                return Err(err.into());
            }
        }
    }
}
