use crate::action::common::{PacketID, TrdHeader};
use crate::action::{
    basic_qot::{
        self,
        get::{GetBasicQotRequest, GetBasicQotResponse},
    },
    global_state::{self, GetGlobalStateRequest, GetGlobalStateResponse},
    init_connect::{self, InitConnectRequest, InitConnectResponse},
    ipo::{self, GetIpoListRequest, GetIpoListResponse},
    max_trd_qtys::{self, GetMaxTrdQtysRequest, GetMaxTrdQtysResponse},
    order::{
        self,
        place::{PlaceOrderRequest, PlaceOrderResponse},
    },
    plate_security::{self, GetPlateSecurityRequest, GetPlateSecurityResponse},
    price_reminder::{
        self,
        get::{GetPriceReminderRequest, GetPriceReminderResponse},
        set::{SetPriceReminderRequest, SetPriceReminderResponse},
    },
    security_snapshot::{self, GetSecuritySnapshotRequest, GetSecuritySnapshotResponse},
    stock_filter::{self, GetStockFilterRequest, GetStockFilterResponse},
    subscribe::{self, SubscribeRequest},
    unlock::{self, UnlockRequest},
    user_security::{
        self,
        get::{GetUserSecurityRequest, GetUserSecurityResponse},
    },
    user_security_group::{
        self,
        get::{GetUserSecurityGroupRequest, GetUserSecurityGroupResponse},
        modify::ModifyUserSecurityGroupRequest,
    },
};
use crate::Trd_Common::{
    OrderType, SecurityFirm, TimeInForce, TrailType, TrdEnv, TrdMarket, TrdSecMarket, TrdSide,
};
use crate::{serial_no, Connection, Frame};
use md5;
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct QotClient {
    connection: Connection,
}

pub struct TrdClient {
    conn_id: u64,
    connection: Connection,
}

pub async fn qot_connect<T: ToSocketAddrs>(addr: T) -> crate::Result<QotClient> {
    let socket = TcpStream::connect(addr).await.unwrap();

    let connection = Connection::new(socket);

    let mut client = QotClient { connection };
    client.init_connect().await?;

    Ok(client)
}

pub async fn trd_connect<T: ToSocketAddrs>(addr: T) -> crate::Result<TrdClient> {
    let socket = TcpStream::connect(addr).await.unwrap();

    let connection = Connection::new(socket);

    let mut client = TrdClient {
        conn_id: 0,
        connection,
    };

    let init_connect_resp = client.init_connect().await?;
    client.set_conn_id(init_connect_resp.conn_id);

    Ok(client)
}

impl TrdClient {
    fn set_conn_id(&mut self, id: u64) {
        self.conn_id = id;
    }

    async fn init_connect(&mut self) -> crate::Result<InitConnectResponse> {
        let frame = InitConnectRequest::default().into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::InitConnect::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        init_connect::check_response(frame.body)
    }

    pub async fn unlock(&mut self, pwd: String) -> crate::Result<()> {
        let pwd_md5 = format!("{:x}", md5::compute(pwd));
        let unlock_req =
            UnlockRequest::new(pwd_md5, Some(SecurityFirm::SecurityFirm_FutuSecurities));
        let frame = unlock_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Trd_UnlockTrade::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        unlock::check_response(frame.body)
    }

    pub async fn get_max_trd_qtys(
        &mut self,
        get_max_trd_qtys_req: GetMaxTrdQtysRequest,
    ) -> crate::Result<GetMaxTrdQtysResponse> {
        let frame = get_max_trd_qtys_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Trd_GetMaxTrdQtys::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        max_trd_qtys::check_response(frame.body)
    }

    pub async fn place_order(
        &mut self,
        acc_id: u64,
        trd_env: TrdEnv,
        trd_market: TrdMarket,
        trd_side: TrdSide,
        order_type: OrderType,
        code: String,
        qty: f64,
        price: Option<f64>,
        adjust_price: Option<bool>,
        adjust_side_and_limit: Option<f64>,
        sec_market: Option<TrdSecMarket>,
        remark: Option<String>,
        time_in_force: Option<TimeInForce>,
        fill_outside_rth: Option<bool>,
        aux_price: Option<f64>,
        trail_type: Option<TrailType>,
        trail_value: Option<f64>,
        trail_spread: Option<f64>,
    ) -> crate::Result<PlaceOrderResponse> {
        let mut place_order_req = PlaceOrderRequest::default();

        place_order_req.packet_id = PacketID {
            conn_id: self.conn_id,
            serial_no: serial_no(),
        };

        place_order_req.header = TrdHeader {
            trd_env,
            acc_id,
            trd_market,
        };

        place_order_req.trd_side = trd_side;
        place_order_req.order_type = order_type;
        place_order_req.code = code;
        place_order_req.qty = qty;
        place_order_req.price = price;
        place_order_req.adjust_price = adjust_price;
        place_order_req.adjust_side_and_limit = adjust_side_and_limit;
        place_order_req.sec_market = sec_market;
        place_order_req.remark = remark;
        place_order_req.time_in_force = time_in_force;
        place_order_req.fill_outside_rth = fill_outside_rth;
        place_order_req.aux_price = aux_price;
        place_order_req.trail_type = trail_type;
        place_order_req.trail_value = trail_value;
        place_order_req.trail_spread = trail_spread;

        let frame = place_order_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Trd_PlaceOrder::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        order::place::check_response(frame.body)
    }
}

impl QotClient {
    async fn init_connect(&mut self) -> crate::Result<InitConnectResponse> {
        let frame = InitConnectRequest::default().into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::InitConnect::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        init_connect::check_response(frame.body)
    }

    pub async fn get_ipo_list(
        &mut self,
        get_ipo_list_req: GetIpoListRequest,
    ) -> crate::Result<GetIpoListResponse> {
        let frame = get_ipo_list_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetIpoList::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        ipo::check_response(frame.body)
    }

    pub async fn get_security_snapshot(
        &mut self,
        get_security_snapshot_req: GetSecuritySnapshotRequest,
    ) -> crate::Result<GetSecuritySnapshotResponse> {
        let frame = get_security_snapshot_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetSecuritySnapshot::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        security_snapshot::check_response(frame.body)
    }

    pub async fn get_user_security_group(
        &mut self,
        get_user_security_group_req: GetUserSecurityGroupRequest,
    ) -> crate::Result<GetUserSecurityGroupResponse> {
        let frame = get_user_security_group_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetUserSecurityGroup::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        user_security_group::get::check_response(frame.body)
    }

    pub async fn get_user_security(
        &mut self,
        get_user_security_req: GetUserSecurityRequest,
    ) -> crate::Result<GetUserSecurityResponse> {
        let frame = get_user_security_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetUserSecurity::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        user_security::get::check_response(frame.body)
    }

    pub async fn modify_user_security_group(
        &mut self,
        modify_user_security_group_req: ModifyUserSecurityGroupRequest,
    ) -> crate::Result<()> {
        let frame = modify_user_security_group_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_ModifyUserSecurity::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        user_security_group::modify::check_response(frame.body)
    }

    pub async fn get_plate_security(
        &mut self,
        get_plate_security_req: GetPlateSecurityRequest,
    ) -> crate::Result<GetPlateSecurityResponse> {
        let frame = get_plate_security_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetPlateSecurity::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        plate_security::check_response(frame.body)
    }

    pub async fn get_global_state(&mut self) -> crate::Result<GetGlobalStateResponse> {
        let frame = GetGlobalStateRequest.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::GetGlobalState::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        global_state::check_response(frame.body)
    }

    pub async fn get_stock_filter(
        &mut self,
        get_stock_filter_req: GetStockFilterRequest,
    ) -> crate::Result<GetStockFilterResponse> {
        let frame = get_stock_filter_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_StockFilter::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        stock_filter::check_response(frame.body)
    }

    pub async fn subscribe(&mut self, subscribe_req: SubscribeRequest) -> crate::Result<()> {
        let frame = subscribe_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_Sub::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        subscribe::check_response(frame.body)
    }

    pub async fn unsubscribe_all(&mut self) -> crate::Result<()> {
        self.subscribe(SubscribeRequest::new(
            vec![],
            vec![],
            false,
            None,
            vec![],
            None,
            Some(true),
            None,
            None,
        ))
        .await
    }

    pub async fn get_basic_qot(
        &mut self,
        get_basic_qot_req: GetBasicQotRequest,
    ) -> crate::Result<GetBasicQotResponse> {
        let frame = get_basic_qot_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetBasicQot::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        basic_qot::get::check_response(frame.body)
    }

    pub async fn set_price_reminder(
        &mut self,
        set_price_reminder_req: SetPriceReminderRequest,
    ) -> crate::Result<SetPriceReminderResponse> {
        let frame = set_price_reminder_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_SetPriceReminder::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        price_reminder::set::check_response(frame.body)
    }

    pub async fn get_price_reminder(
        &mut self,
        get_price_reminder_req: GetPriceReminderRequest,
    ) -> crate::Result<GetPriceReminderResponse> {
        let frame = get_price_reminder_req.into_frame();
        self.connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetPriceReminder::Response> =
            self.connection.read_frame().await.unwrap().unwrap();
        price_reminder::get::check_response(frame.body)
    }
}
