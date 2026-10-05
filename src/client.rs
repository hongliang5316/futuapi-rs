use crate::{
    action::{
        basic_qot::{
            self,
            get::{GetBasicQotRequest, GetBasicQotResponse},
            update::UpdateBasicQotResponse,
        },
        common::{PacketID, TrdHeader},
        global_state::{self, GetGlobalStateRequest, GetGlobalStateResponse},
        history_order_list::{self, GetHistoryOrderListRequest, GetHistoryOrderListResponse},
        init_connect::{self, InitConnectRequest, InitConnectResponse},
        ipo::{self, GetIpoListRequest, GetIpoListResponse},
        keepalive::KeepAliveRequest,
        kl::{self, update::UpdateKLResponse},
        max_trd_qtys::{self, GetMaxTrdQtysRequest, GetMaxTrdQtysResponse},
        order::{
            self,
            modify::{ModifyOrderRequest, ModifyOrderResponse},
            place::{PlaceOrderRequest, PlaceOrderResponse},
        },
        plate_security::{self, GetPlateSecurityRequest, GetPlateSecurityResponse},
        position_list::{self, GetPositionListRequest, GetPositionListResponse},
        price_reminder::{
            self,
            get::{GetPriceReminderRequest, GetPriceReminderResponse},
            set::{SetPriceReminderRequest, SetPriceReminderResponse},
        },
        request_history_kl::{self, RequestHistoryKLRequest, RequestHistoryKLResponse},
        rt::{self, update::UpdateRTResponse},
        security_snapshot::{self, GetSecuritySnapshotRequest, GetSecuritySnapshotResponse},
        stock_filter::{self, GetStockFilterRequest, GetStockFilterResponse},
        subscribe::{self, SubscribeRequest},
        unlock::{self, UnlockRequest},
        user_security::{
            self,
            get::{GetUserSecurityRequest, GetUserSecurityResponse},
            modify::ModifyUserSecurityRequest,
        },
        user_security_group::{
            self,
            get::{GetUserSecurityGroupRequest, GetUserSecurityGroupResponse},
        },
    },
    connection::FrameReader,
    frame::{self, FrameRaw},
    serial_no, Connection, Frame,
    Trd_Common::{
        ModifyOrderOp, OrderType, SecurityFirm, TimeInForce, TrailType, TrdEnv, TrdMarket,
        TrdSecMarket, TrdSide,
    },
};
use protobuf::MessageFull;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    sync::{Arc, Mutex as StdMutex},
};
use tokio::{
    io::AsyncWriteExt,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::{sleep, Duration},
};

pub struct QotClient {
    connection: Connection,
}

pub struct TrdClient {
    conn_id: u64,
    connection: Connection,
}

/// Request side of a subscription connection. Cheap to clone, so subscriptions
/// can be changed from any task while a `Subscriber` is reading pushes.
#[derive(Clone)]
pub struct SubClient {
    requester: Requester,
    _tasks: Arc<TaskGuard>,
}

pub async fn qot_connect<T: ToSocketAddrs>(addr: T) -> crate::Result<QotClient> {
    let socket = TcpStream::connect(addr).await?;
    let connection = Connection::new(socket);

    let mut client = QotClient { connection };
    client.init_connect().await?;

    Ok(client)
}

/// Connects a subscription connection. The socket is split: a reader task routes
/// responses back to their request and everything else to the `Subscriber`, a
/// writer task owns the write half, and a keepalive task runs independently of both.
pub async fn sub_connect<T: ToSocketAddrs>(addr: T) -> crate::Result<(SubClient, Subscriber)> {
    let socket = TcpStream::connect(addr).await?;
    let (read_half, write_half) = socket.into_split();

    let pending = Arc::new(StdMutex::new(Some(HashMap::new())));
    let (write_tx, write_rx) = mpsc::unbounded_channel();
    // Unbounded so a slow consumer never blocks the reader, which would also
    // hold up the response to a `subscribe` awaited on the consumer's task.
    let (push_tx, push_rx) = mpsc::unbounded_channel();

    let mut tasks = TaskGuard(vec![
        tokio::spawn(read_loop(
            FrameReader::new(read_half),
            pending.clone(),
            push_tx,
        )),
        tokio::spawn(write_loop(write_half, write_rx)),
    ]);

    let requester = Requester { write_tx, pending };
    let frame: Frame<crate::InitConnect::Response> = requester
        .request(InitConnectRequest::default().into_frame())
        .await?;
    let init_connect_resp = init_connect::check_response(frame.body)?;

    tasks.0.push(tokio::spawn(keepalive_loop(
        requester.write_tx.clone(),
        init_connect_resp.keep_alive_interval,
    )));

    let tasks = Arc::new(tasks);
    let client = SubClient {
        requester,
        _tasks: tasks.clone(),
    };
    let subscriber = Subscriber {
        push_rx,
        _tasks: tasks,
    };

    Ok((client, subscriber))
}

pub async fn trd_connect<T: ToSocketAddrs>(addr: T) -> crate::Result<TrdClient> {
    let socket = TcpStream::connect(addr).await?;
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
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::InitConnect::Response> = match self.connection.read_frame().await? {
            Some(frame) => frame,
            None => {
                let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                return Err(err.into());
            }
        };
        init_connect::check_response(frame.body)
    }

    pub async fn unlock(&mut self, pwd: String) -> crate::Result<()> {
        let pwd_md5 = format!("{:x}", md5::compute(pwd));
        let unlock_req =
            UnlockRequest::new(pwd_md5, Some(SecurityFirm::SecurityFirm_FutuSecurities));
        let frame = unlock_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Trd_UnlockTrade::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        unlock::check_response(frame.body)
    }

    pub async fn get_max_trd_qtys(
        &mut self,
        get_max_trd_qtys_req: GetMaxTrdQtysRequest,
    ) -> crate::Result<GetMaxTrdQtysResponse> {
        let frame = get_max_trd_qtys_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Trd_GetMaxTrdQtys::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        max_trd_qtys::check_response(frame.body)
    }

    pub async fn get_position_list(
        &mut self,
        get_position_list_req: GetPositionListRequest,
    ) -> crate::Result<GetPositionListResponse> {
        let frame = get_position_list_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Trd_GetPositionList::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        position_list::check_response(frame.body)
    }

    pub async fn get_history_order_list(
        &mut self,
        get_history_order_list_req: GetHistoryOrderListRequest,
    ) -> crate::Result<GetHistoryOrderListResponse> {
        let frame = get_history_order_list_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Trd_GetHistoryOrderList::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        history_order_list::check_response(frame.body)
    }

    pub async fn modify_order(
        &mut self,
        acc_id: u64,
        trd_env: TrdEnv,
        trd_market: TrdMarket,
        order_id: u64,
        modify_order_op: ModifyOrderOp,
        qty: Option<f64>,
        price: Option<f64>,
    ) -> crate::Result<ModifyOrderResponse> {
        let modify_order_req = ModifyOrderRequest::new(
            PacketID {
                conn_id: self.conn_id,
                serial_no: serial_no(),
            },
            TrdHeader {
                acc_id,
                trd_env,
                trd_market,
            },
            order_id,
            modify_order_op,
            qty,
            price,
        );
        let frame = modify_order_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Trd_ModifyOrder::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        order::modify::check_response(frame.body)
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
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Trd_PlaceOrder::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        order::place::check_response(frame.body)
    }
}

impl SubClient {
    pub async fn subscribe(&self, subscribe_req: SubscribeRequest) -> crate::Result<()> {
        let frame: Frame<crate::Qot_Sub::Response> =
            self.requester.request(subscribe_req.into_frame()).await?;
        subscribe::check_response(frame.body)
    }

    pub async fn unsubscribe_all(&self) -> crate::Result<()> {
        let subscribe_req = SubscribeRequest::new(
            vec![],
            vec![],
            false,
            None,
            vec![],
            None,
            Some(true),
            None,
            None,
        );

        self.subscribe(subscribe_req).await
    }
}

/// In-flight requests keyed by `(proto_id, serial_no)`, which OpenD echoes back
/// in the response header. `None` once the reader has stopped.
type Pending = StdMutex<Option<HashMap<(u32, u32), oneshot::Sender<FrameRaw>>>>;

#[derive(Clone)]
struct Requester {
    write_tx: mpsc::UnboundedSender<Vec<u8>>,
    pending: Arc<Pending>,
}

impl Requester {
    async fn request<Req: MessageFull, Resp: MessageFull>(
        &self,
        frame: Frame<Req>,
    ) -> crate::Result<Frame<Resp>> {
        let key = (frame.header.proto_id, frame.header.serial_no);
        let (tx, rx) = oneshot::channel();
        match self.pending.lock().unwrap().as_mut() {
            Some(pending) => pending.insert(key, tx),
            None => return Err(connection_reset()),
        };
        let _guard = PendingGuard {
            pending: &self.pending,
            key,
        };

        self.write_tx
            .send(frame.to_bytes()?)
            .map_err(|_| connection_reset())?;
        let frame_raw = rx.await.map_err(|_| connection_reset())?;
        Ok(Frame::from_raw(frame_raw)?)
    }
}

/// Removes the pending entry when a request fails or is cancelled before its
/// response arrives.
struct PendingGuard<'a> {
    pending: &'a Pending,
    key: (u32, u32),
}

impl Drop for PendingGuard<'_> {
    fn drop(&mut self) {
        if let Some(pending) = self.pending.lock().unwrap().as_mut() {
            pending.remove(&self.key);
        }
    }
}

/// Aborts the background tasks once both `SubClient` and `Subscriber` are gone.
struct TaskGuard(Vec<JoinHandle<()>>);

impl Drop for TaskGuard {
    fn drop(&mut self) {
        for handle in &self.0 {
            handle.abort();
        }
    }
}

fn connection_reset() -> crate::Error {
    Error::new(ErrorKind::ConnectionReset, "connection reset by server").into()
}

async fn read_loop(
    mut reader: FrameReader<OwnedReadHalf>,
    pending: Arc<Pending>,
    push_tx: mpsc::UnboundedSender<Result<FrameRaw, frame::Error>>,
) {
    loop {
        match reader.read_frame_raw().await {
            Ok(Some(frame_raw)) => {
                let key = (frame_raw.header.proto_id, frame_raw.header.serial_no);
                let waiter = pending
                    .lock()
                    .unwrap()
                    .as_mut()
                    .and_then(|pending| pending.remove(&key));
                // anything that doesn't answer an in-flight request is a push
                match waiter {
                    Some(waiter) => {
                        let _ = waiter.send(frame_raw);
                    }
                    None => {
                        let _ = push_tx.send(Ok(frame_raw));
                    }
                }
            }
            Ok(None) => break,
            Err(e) => {
                let _ = push_tx.send(Err(e));
                break;
            }
        }
    }

    // fail in-flight and later requests instead of leaving them waiting forever
    *pending.lock().unwrap() = None;
}

async fn write_loop(mut writer: OwnedWriteHalf, mut write_rx: mpsc::UnboundedReceiver<Vec<u8>>) {
    while let Some(buf) = write_rx.recv().await {
        if writer.write_all(&buf).await.is_err() {
            break;
        }
    }
}

async fn keepalive_loop(write_tx: mpsc::UnboundedSender<Vec<u8>>, keep_alive_interval: i32) {
    let interval = Duration::from_secs(keep_alive_interval.max(1) as u64);
    loop {
        sleep(interval).await;
        // the reply has no pending entry, so it reaches `next_data` and is skipped there
        let frame = KeepAliveRequest::new(chrono::Local::now().timestamp()).into_frame();
        let Ok(buf) = frame.to_bytes() else { break };
        if write_tx.send(buf).is_err() {
            break;
        }
    }
}

impl QotClient {
    async fn init_connect(&mut self) -> crate::Result<InitConnectResponse> {
        let frame = InitConnectRequest::default().into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::InitConnect::Response> = match self.connection.read_frame().await? {
            Some(frame) => frame,
            None => {
                let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                return Err(err.into());
            }
        };
        init_connect::check_response(frame.body)
    }

    pub async fn get_ipo_list(
        &mut self,
        get_ipo_list_req: GetIpoListRequest,
    ) -> crate::Result<GetIpoListResponse> {
        let frame = get_ipo_list_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_GetIpoList::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        ipo::check_response(frame.body)
    }

    pub async fn get_security_snapshot(
        &mut self,
        get_security_snapshot_req: GetSecuritySnapshotRequest,
    ) -> crate::Result<GetSecuritySnapshotResponse> {
        let frame = get_security_snapshot_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_GetSecuritySnapshot::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        security_snapshot::check_response(frame.body)
    }

    pub async fn request_history_kl(
        &mut self,
        request_history_kl_req: RequestHistoryKLRequest,
    ) -> crate::Result<RequestHistoryKLResponse> {
        let frame = request_history_kl_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_RequestHistoryKL::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        request_history_kl::check_response(frame.body)
    }

    pub async fn get_user_security_group(
        &mut self,
        get_user_security_group_req: GetUserSecurityGroupRequest,
    ) -> crate::Result<GetUserSecurityGroupResponse> {
        let frame = get_user_security_group_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_GetUserSecurityGroup::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        user_security_group::get::check_response(frame.body)
    }

    pub async fn get_user_security(
        &mut self,
        get_user_security_req: GetUserSecurityRequest,
    ) -> crate::Result<GetUserSecurityResponse> {
        let frame = get_user_security_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_GetUserSecurity::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        user_security::get::check_response(frame.body)
    }

    pub async fn modify_user_security(
        &mut self,
        modify_user_security_req: ModifyUserSecurityRequest,
    ) -> crate::Result<()> {
        let frame = modify_user_security_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_ModifyUserSecurity::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        user_security::modify::check_response(frame.body)
    }

    pub async fn get_plate_security(
        &mut self,
        get_plate_security_req: GetPlateSecurityRequest,
    ) -> crate::Result<GetPlateSecurityResponse> {
        let frame = get_plate_security_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_GetPlateSecurity::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        plate_security::check_response(frame.body)
    }

    pub async fn get_global_state(&mut self) -> crate::Result<GetGlobalStateResponse> {
        let frame = GetGlobalStateRequest.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::GetGlobalState::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        global_state::check_response(frame.body)
    }

    pub async fn get_stock_filter(
        &mut self,
        get_stock_filter_req: GetStockFilterRequest,
    ) -> crate::Result<GetStockFilterResponse> {
        let frame = get_stock_filter_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_StockFilter::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        stock_filter::check_response(frame.body)
    }

    pub async fn get_basic_qot(
        &mut self,
        get_basic_qot_req: GetBasicQotRequest,
    ) -> crate::Result<GetBasicQotResponse> {
        let frame = get_basic_qot_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_GetBasicQot::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        basic_qot::get::check_response(frame.body)
    }

    pub async fn set_price_reminder(
        &mut self,
        set_price_reminder_req: SetPriceReminderRequest,
    ) -> crate::Result<SetPriceReminderResponse> {
        let frame = set_price_reminder_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_SetPriceReminder::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        price_reminder::set::check_response(frame.body)
    }

    pub async fn get_price_reminder(
        &mut self,
        get_price_reminder_req: GetPriceReminderRequest,
    ) -> crate::Result<GetPriceReminderResponse> {
        let frame = get_price_reminder_req.into_frame();
        self.connection.write_frame(&frame).await?;
        let frame: Frame<crate::Qot_GetPriceReminder::Response> =
            match self.connection.read_frame().await? {
                Some(frame) => frame,
                None => {
                    let err = Error::new(ErrorKind::ConnectionReset, "connection reset by server");
                    return Err(err.into());
                }
            };
        price_reminder::get::check_response(frame.body)
    }
}

pub struct Subscriber {
    push_rx: mpsc::UnboundedReceiver<Result<FrameRaw, frame::Error>>,
    _tasks: Arc<TaskGuard>,
}

#[derive(Debug)]
pub enum UpdateResponse {
    BasicQot(UpdateBasicQotResponse),
    RT(UpdateRTResponse),
    KL(UpdateKLResponse),
}

impl Subscriber {
    /// Waits for the next push. `Err` means this push (or the connection) failed;
    /// `Ok(None)` means the connection is closed and no more pushes will come.
    pub async fn next_data(&mut self) -> crate::Result<Option<UpdateResponse>> {
        loop {
            let frame_raw = match self.push_rx.recv().await {
                Some(frame_raw) => frame_raw?,
                None => return Ok(None),
            };

            let resp = match frame_raw.header.proto_id {
                basic_qot::update::PROTO_ID => {
                    let frame: Frame<crate::Qot_UpdateBasicQot::Response> =
                        Frame::from_raw(frame_raw)?;
                    UpdateResponse::BasicQot(basic_qot::update::check_response(frame.body)?)
                }
                rt::update::PROTO_ID => {
                    let frame: Frame<crate::Qot_UpdateRT::Response> = Frame::from_raw(frame_raw)?;
                    UpdateResponse::RT(rt::update::check_response(frame.body)?)
                }
                kl::update::PROTO_ID => {
                    let frame: Frame<crate::Qot_UpdateKL::Response> = Frame::from_raw(frame_raw)?;
                    UpdateResponse::KL(kl::update::check_response(frame.body)?)
                }
                // keepalive replies and pushes not handled yet
                _ => continue,
            };

            return Ok(Some(resp));
        }
    }
}
