use futuapi_rs::action::ipo::GetIpoListRequest;
use futuapi_rs::action::subscribe::SubscribeRequest;
use futuapi_rs::client;
use futuapi_rs::Connection;
use futuapi_rs::Qot_Common::SubType;
use futuapi_rs::Result;
use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::qot_connect("127.0.0.1:11111").await?;
    client
        .subscribe(SubscribeRequest::new(
            // vec!["US.CEI".try_into().unwrap(), "US.SKYX".try_into().unwrap()],
            vec!["HK.00700".try_into().unwrap()],
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

    let rx = client.connection.receiver.clone();
    let handle = tokio::spawn(async move {
        println!("1111111111111");
        while let Some(i) = rx.lock().await.recv().await {
            println!("got = {:?}", i);
        }
    });

    // Do some other work

    // handle.await.unwrap();

    loop {
        println!("start start start");
        let ipo_list = client
            .get_ipo_list(GetIpoListRequest::new(
                futuapi_rs::Qot_Common::QotMarket::QotMarket_US_Security,
            ))
            .await?;

        println!("ipo_list {:?}", ipo_list);

        sleep(Duration::from_secs(3)).await;
    }

    //    Ok(())
}
