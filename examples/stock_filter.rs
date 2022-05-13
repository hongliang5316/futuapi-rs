use futuapi_rs::action::stock_filter::{AccumulateFilter, BaseFilter, GetStockFilterRequest};
use futuapi_rs::Qot_Common::QotMarket;
use futuapi_rs::Qot_StockFilter::{AccumulateField, SortDir, StockField};
use futuapi_rs::{client, Result};
use std::collections::HashMap;

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::qot_connect("127.0.0.1:11111").await?;
    let code_info: Vec<_> = client
        .get_stock_filter(GetStockFilterRequest::new(
            0,
            10,
            QotMarket::QotMarket_US_Security,
            None,
            vec![BaseFilter::new(
                StockField::StockField_VolumeRatio,
                Some(3.0), // min
                None,      // max
                Some(false),
                None, // sort
            )],
            vec![AccumulateFilter::new(
                AccumulateField::AccumulateField_ChangeRate,
                Some(4.0), // min
                None,      // max
                Some(false),
                Some(SortDir::SortDir_Descend),
                1,
            )],
        ))
        .await?
        .data_list
        .iter()
        .map(|stock_data| {
            let mut base_field_map = HashMap::new();
            for base_data in &stock_data.base_data_list {
                base_field_map.insert(base_data.field_name, base_data.value);
            }

            let mut accumulate_field_map = HashMap::new();
            for accumulate_data in &stock_data.accumulate_data_list {
                accumulate_field_map.insert(accumulate_data.field_name, accumulate_data.value);
            }

            (
                stock_data.security.to_string(),
                base_field_map,
                accumulate_field_map,
            )
        })
        .collect();

    println!("{:?}", code_info);

    Ok(())
}
