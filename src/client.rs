use crate::action::{
    basic_qot::{
        self,
        get::{GetBasicQotRequest, GetBasicQotResponse},
    },
    global_state::{self, GetGlobalStateRequest, GetGlobalStateResponse},
    init_connect::{self, InitConnectRequest, InitConnectResponse},
    ipo::{self, GetIpoListRequest, GetIpoListResponse},
    plate_security::{self, GetPlateSecurityRequest, GetPlateSecurityResponse},
    security_snapshot::{self, GetSecuritySnapshotRequest, GetSecuritySnapshotResponse},
    stock_filter::{self, GetStockFilterRequest, GetStockFilterResponse},
    subscribe::{self, SubscribeRequest},
    user_security_group::{
        self,
        get::{GetUserSecurityGroupRequest, GetUserSecurityGroupResponse},
        modify::ModifyUserSecurityGroupRequest,
    },
};
use crate::{Connection, Frame};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct QotClient {
    connection: Connection,
}

// pub struct TrdClient {
//     connection: Connection,
// }

pub async fn qot_connect<T: ToSocketAddrs>(addr: T) -> crate::Result<QotClient> {
    let socket = TcpStream::connect(addr).await.unwrap();

    let connection = Connection::new(socket);

    let mut client = QotClient { connection };
    client.init_connect().await?;

    Ok(client)
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
}
