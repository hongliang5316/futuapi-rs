use futuapi_rs::{
    action::{
        common::{TrdFilterConditions, TrdHeader},
        history_order_list::GetHistoryOrderListRequest,
    },
    client, Result,
    Trd_Common::{OrderStatus, TrdEnv, TrdMarket},
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut trd_client = client::trd_connect("127.0.0.1:11111").await?;

    let resp = trd_client
        .get_history_order_list(GetHistoryOrderListRequest::new(
            TrdHeader {
                trd_env: TrdEnv::TrdEnv_Real,
                acc_id: 0,
                trd_market: TrdMarket::TrdMarket_US,
            },
            TrdFilterConditions {
                code_list: Vec::new(),
                id_list: Vec::new(),
                begin_time: None,
                end_time: None,
            },
            vec![
                OrderStatus::OrderStatus_Unsubmitted,
                OrderStatus::OrderStatus_Unknown,
                OrderStatus::OrderStatus_WaitingSubmit,
                OrderStatus::OrderStatus_Submitting,
                OrderStatus::OrderStatus_Submitted,
                OrderStatus::OrderStatus_Filled_Part,
            ],
        ))
        .await?;

    println!("{:?}", resp);

    Ok(())
}
