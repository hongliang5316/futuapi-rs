use super::common::Security;
use crate::Common::RetType;
use crate::Frame;
use crate::Qot_Common::QotMarket;
use crate::Qot_StockFilter::{self, AccumulateField, Request, Response, SortDir, StockField, C2S};
use protobuf::{Enum, MessageField};

const PROTO_ID: u32 = 3215;

pub struct BaseFilter {
    field_name: StockField,
    filter_min: Option<f64>,
    filter_max: Option<f64>,
    is_no_filter: Option<bool>,
    sort_dir: Option<SortDir>,
}

impl BaseFilter {
    pub fn new(
        field_name: StockField,
        filter_min: Option<f64>,
        filter_max: Option<f64>,
        is_no_filter: Option<bool>,
        sort_dir: Option<SortDir>,
    ) -> Self {
        BaseFilter {
            field_name,
            filter_min,
            filter_max,
            is_no_filter,
            sort_dir,
        }
    }
}

pub struct BaseFilterVec(Vec<BaseFilter>);

impl Into<Vec<Qot_StockFilter::BaseFilter>> for BaseFilterVec {
    fn into(self) -> Vec<Qot_StockFilter::BaseFilter> {
        let mut base_filter_list = Vec::new();
        for base_filter in self.0 {
            base_filter_list.push(base_filter.into());
        }

        base_filter_list
    }
}

impl Into<Qot_StockFilter::BaseFilter> for BaseFilter {
    fn into(self) -> Qot_StockFilter::BaseFilter {
        let mut base_filter = Qot_StockFilter::BaseFilter::new();
        base_filter.set_fieldName(self.field_name as i32);
        base_filter.filterMin = self.filter_min;
        base_filter.filterMax = self.filter_max;
        base_filter.isNoFilter = self.is_no_filter;
        base_filter.sortDir = if self.sort_dir.is_some() {
            Some(self.sort_dir.unwrap() as i32)
        } else {
            None
        };

        base_filter
    }
}

pub struct AccumulateFilter {
    field_name: AccumulateField,
    filter_min: Option<f64>,
    filter_max: Option<f64>,
    is_no_filter: Option<bool>,
    sort_dir: Option<SortDir>,
    days: i32,
}

impl AccumulateFilter {
    pub fn new(
        field_name: AccumulateField,
        filter_min: Option<f64>,
        filter_max: Option<f64>,
        is_no_filter: Option<bool>,
        sort_dir: Option<SortDir>,
        days: i32,
    ) -> Self {
        AccumulateFilter {
            field_name,
            filter_min,
            filter_max,
            is_no_filter,
            sort_dir,
            days,
        }
    }
}

pub struct AccumulateFilterVec(Vec<AccumulateFilter>);

impl Into<Vec<Qot_StockFilter::AccumulateFilter>> for AccumulateFilterVec {
    fn into(self) -> Vec<Qot_StockFilter::AccumulateFilter> {
        let mut accumulate_filter_list = Vec::new();
        for accumulate_filter in self.0 {
            accumulate_filter_list.push(accumulate_filter.into())
        }

        accumulate_filter_list
    }
}

impl Into<Qot_StockFilter::AccumulateFilter> for AccumulateFilter {
    fn into(self) -> Qot_StockFilter::AccumulateFilter {
        let mut accumulate_filter = Qot_StockFilter::AccumulateFilter::new();
        accumulate_filter.set_fieldName(self.field_name as i32);
        accumulate_filter.filterMin = self.filter_min;
        accumulate_filter.filterMax = self.filter_max;
        accumulate_filter.isNoFilter = self.is_no_filter;
        accumulate_filter.sortDir = if self.sort_dir.is_some() {
            Some(self.sort_dir.unwrap() as i32)
        } else {
            None
        };
        accumulate_filter.set_days(self.days);

        accumulate_filter
    }
}

pub struct GetStockFilterRequest {
    begin: i32,
    num: i32,
    market: QotMarket,
    plate: Option<Security>,
    base_filter_list: Vec<BaseFilter>,
    accumulate_filter_list: Vec<AccumulateFilter>,
}

impl Into<Request> for GetStockFilterRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_begin(self.begin);
        c2s.set_num(self.num);
        c2s.set_market(self.market as i32);

        if self.plate.is_some() {
            c2s.plate = MessageField::some(self.plate.unwrap().into());
        }

        c2s.baseFilterList = BaseFilterVec(self.base_filter_list).into();
        c2s.accumulateFilterList = AccumulateFilterVec(self.accumulate_filter_list).into();

        req.c2s = MessageField::some(c2s);

        req
    }
}

impl GetStockFilterRequest {
    pub fn new(
        begin: i32,
        num: i32,
        market: QotMarket,
        plate: Option<Security>,
        base_filter_list: Vec<BaseFilter>,
        accumulate_filter_list: Vec<AccumulateFilter>,
    ) -> Self {
        GetStockFilterRequest {
            begin,
            num,
            market,
            plate,
            base_filter_list,
            accumulate_filter_list,
        }
    }

    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

#[derive(Debug)]
pub struct BaseData {
    pub field_name: StockField,
    pub value: f64,
}

impl From<Qot_StockFilter::BaseData> for BaseData {
    fn from(base_data: Qot_StockFilter::BaseData) -> Self {
        BaseData {
            field_name: StockField::from_i32(base_data.fieldName()).unwrap(),
            value: base_data.value(),
        }
    }
}

pub struct BaseDataVec(Vec<BaseData>);

impl From<Vec<Qot_StockFilter::BaseData>> for BaseDataVec {
    fn from(qot_base_data_list: Vec<Qot_StockFilter::BaseData>) -> Self {
        let mut base_data_list = Vec::new();
        for base_data in qot_base_data_list {
            base_data_list.push(base_data.into());
        }

        BaseDataVec(base_data_list)
    }
}

#[derive(Debug)]
pub struct AccumulateData {
    pub field_name: AccumulateField,
    pub value: f64,
    pub days: i32,
}

impl From<Qot_StockFilter::AccumulateData> for AccumulateData {
    fn from(accumulate_data: Qot_StockFilter::AccumulateData) -> AccumulateData {
        AccumulateData {
            field_name: AccumulateField::from_i32(accumulate_data.fieldName()).unwrap(),
            value: accumulate_data.value(),
            days: accumulate_data.days(),
        }
    }
}

pub struct AccumulateDataVec(Vec<AccumulateData>);

impl From<Vec<Qot_StockFilter::AccumulateData>> for AccumulateDataVec {
    fn from(qot_accumulate_data_list: Vec<Qot_StockFilter::AccumulateData>) -> AccumulateDataVec {
        let mut accumulate_data_list = Vec::new();
        for accumulate_data in qot_accumulate_data_list {
            accumulate_data_list.push(accumulate_data.into());
        }

        AccumulateDataVec(accumulate_data_list)
    }
}

#[derive(Debug)]
pub struct StockData {
    pub security: Security,
    pub name: String,
    pub base_data_list: Vec<BaseData>,
    pub accumulate_data_list: Vec<AccumulateData>,
}

impl From<Qot_StockFilter::StockData> for StockData {
    fn from(stock_data: Qot_StockFilter::StockData) -> Self {
        StockData {
            security: stock_data.security.to_owned().unwrap().into(),
            name: stock_data.name().into(),
            base_data_list: BaseDataVec::from(stock_data.baseDataList).0,
            accumulate_data_list: AccumulateDataVec::from(stock_data.accumulateDataList).0,
        }
    }
}

pub struct StockDataVec(Vec<StockData>);

impl From<Vec<Qot_StockFilter::StockData>> for StockDataVec {
    fn from(qot_stock_data_list: Vec<Qot_StockFilter::StockData>) -> Self {
        let mut stock_data_list = Vec::new();
        for stock_data in qot_stock_data_list {
            stock_data_list.push(stock_data.into());
        }

        StockDataVec(stock_data_list)
    }
}

#[derive(Debug)]
pub struct GetStockFilterResponse {
    pub last_page: bool,
    pub all_count: i32,
    pub data_list: Vec<StockData>,
}

impl From<Response> for GetStockFilterResponse {
    fn from(resp: Response) -> Self {
        GetStockFilterResponse {
            last_page: resp.s2c.lastPage(),
            all_count: resp.s2c.allCount(),
            data_list: StockDataVec::from(resp.s2c.dataList.to_owned()).0,
        }
    }
}

pub fn check_response(resp: Response) -> crate::Result<GetStockFilterResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(resp.into());
    }

    Err(resp.retMsg().into())
}
