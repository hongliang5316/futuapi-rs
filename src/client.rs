use crate::action::{
    basic_qot::{
        self,
        get::{GetBasicQotRequest, GetBasicQotResponse},
        update::UpdateData,
    },
    global_state::{self, GetGlobalStateRequest, GetGlobalStateResponse},
    init_connect::{self, InitConnectRequest, InitConnectResponse},
    ipo::{self, GetIpoListRequest, GetIpoListResponse},
    plate_security::{self, GetPlateSecurityRequest, GetPlateSecurityResponse},
    price_reminder::{
        self,
        get::{GetPriceReminderRequest, GetPriceReminderResponse},
        set::{SetPriceReminderRequest, SetPriceReminderResponse},
    },
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
use std::sync::Arc;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct QotClient {
    connection: Arc<Mutex<Connection>>,
    update_handler: Option<JoinHandle<()>>,
}

// pub struct TrdClient {
//     connection: Connection,
// }

pub async fn qot_connect<T: ToSocketAddrs>(addr: T) -> crate::Result<QotClient> {
    let socket = TcpStream::connect(addr).await.unwrap();

    let connection = Connection::new(socket);

    let mut client = QotClient {
        connection: Arc::new(Mutex::new(connection)),
        update_handler: None,
    };

    client.init_connect().await?;

    Ok(client)
}

impl QotClient {
    async fn init_connect(&mut self) -> crate::Result<InitConnectResponse> {
        let frame = InitConnectRequest::default().into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::InitConnect::Response> = connection
            .read_frame(init_connect::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        init_connect::check_response(frame.body)
    }

    pub async fn get_ipo_list(
        &mut self,
        get_ipo_list_req: GetIpoListRequest,
    ) -> crate::Result<GetIpoListResponse> {
        let frame = get_ipo_list_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetIpoList::Response> =
            connection.read_frame(ipo::PROTO_ID).await.unwrap().unwrap();
        ipo::check_response(frame.body)
    }

    pub async fn get_security_snapshot(
        &mut self,
        get_security_snapshot_req: GetSecuritySnapshotRequest,
    ) -> crate::Result<GetSecuritySnapshotResponse> {
        let frame = get_security_snapshot_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetSecuritySnapshot::Response> = connection
            .read_frame(security_snapshot::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        security_snapshot::check_response(frame.body)
    }

    pub async fn get_user_security_group(
        &mut self,
        get_user_security_group_req: GetUserSecurityGroupRequest,
    ) -> crate::Result<GetUserSecurityGroupResponse> {
        let frame = get_user_security_group_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetUserSecurityGroup::Response> = connection
            .read_frame(user_security_group::get::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        user_security_group::get::check_response(frame.body)
    }

    pub async fn modify_user_security_group(
        &mut self,
        modify_user_security_group_req: ModifyUserSecurityGroupRequest,
    ) -> crate::Result<()> {
        let frame = modify_user_security_group_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_ModifyUserSecurity::Response> = connection
            .read_frame(user_security_group::modify::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        user_security_group::modify::check_response(frame.body)
    }

    pub async fn get_plate_security(
        &mut self,
        get_plate_security_req: GetPlateSecurityRequest,
    ) -> crate::Result<GetPlateSecurityResponse> {
        let frame = get_plate_security_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetPlateSecurity::Response> = connection
            .read_frame(plate_security::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        plate_security::check_response(frame.body)
    }

    pub async fn get_global_state(&mut self) -> crate::Result<GetGlobalStateResponse> {
        let frame = GetGlobalStateRequest.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::GetGlobalState::Response> = connection
            .read_frame(global_state::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        global_state::check_response(frame.body)
    }

    pub async fn get_stock_filter(
        &mut self,
        get_stock_filter_req: GetStockFilterRequest,
    ) -> crate::Result<GetStockFilterResponse> {
        let frame = get_stock_filter_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_StockFilter::Response> = connection
            .read_frame(stock_filter::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        stock_filter::check_response(frame.body)
    }

    pub async fn subscribe(&mut self, subscribe_req: SubscribeRequest) -> crate::Result<()> {
        let mut updated = false;
        if subscribe_req.is_sub_or_un_sub && subscribe_req.security_list.len() > 0 {
            if let Some(true) = subscribe_req.is_reg_or_un_reg_push {
                updated = true;
            }
        }

        let frame = subscribe_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_Sub::Response> = connection
            .read_frame(subscribe::PROTO_ID)
            .await
            .unwrap()
            .unwrap();

        if updated {
            let mut client = QotClient {
                connection: self.connection.clone(),
                update_handler: None,
            };
            let handler: JoinHandle<()> = tokio::spawn(async move {
                loop {
                    println!("start update");
                    client.update().await;
                }
            });

            self.update_handler = Some(handler);
        }

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
        .await?;

        self.update_handler.as_ref().unwrap().abort();
        Ok(())
    }

    pub async fn get_basic_qot(
        &mut self,
        get_basic_qot_req: GetBasicQotRequest,
    ) -> crate::Result<GetBasicQotResponse> {
        let frame = get_basic_qot_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetBasicQot::Response> = connection
            .read_frame(basic_qot::get::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        basic_qot::get::check_response(frame.body)
    }

    pub async fn update(&mut self) {
        let mut connection = self.connection.lock().await;
        let frame: Frame<crate::Qot_GetBasicQot::Response> =
            connection.read_frame(0).await.unwrap().unwrap();

        if let Some(update_data) = frame.other {
            connection.send_data(update_data);
        }
    }

    pub async fn set_price_reminder(
        &mut self,
        set_price_reminder_req: SetPriceReminderRequest,
    ) -> crate::Result<SetPriceReminderResponse> {
        let frame = set_price_reminder_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_SetPriceReminder::Response> = connection
            .read_frame(price_reminder::set::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        price_reminder::set::check_response(frame.body)
    }

    pub async fn get_price_reminder(
        &mut self,
        get_price_reminder_req: GetPriceReminderRequest,
    ) -> crate::Result<GetPriceReminderResponse> {
        let frame = get_price_reminder_req.into_frame();
        let mut connection = self.connection.lock().await;
        connection.write_frame(&frame).await.unwrap();
        let frame: Frame<crate::Qot_GetPriceReminder::Response> = connection
            .read_frame(price_reminder::get::PROTO_ID)
            .await
            .unwrap()
            .unwrap();
        price_reminder::get::check_response(frame.body)
    }
}
