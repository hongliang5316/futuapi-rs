use futuapi_rs::{
    action::subscribe::SubscribeRequest, client, connection::FrameReader, frame::FrameRaw, Frame,
    Qot_Common::SubType, UpdateResponse,
};
use protobuf::{MessageField, MessageFull};
use tokio::{
    io::AsyncWriteExt,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    time::{timeout, Duration},
};

const INIT_CONNECT: u32 = 1001;
const KEEPALIVE: u32 = 1004;
const SUB: u32 = 3001;
const UPDATE_BASIC_QOT: u32 = 3005;

fn sub_req() -> SubscribeRequest {
    SubscribeRequest::new(
        vec!["HK.00700".try_into().unwrap()],
        vec![SubType::SubType_Basic],
        true,
        Some(true),
        vec![],
        None,
        None,
        None,
        None,
    )
}

async fn send<T: MessageFull>(w: &mut OwnedWriteHalf, body: T, proto_id: u32, serial_no: u32) {
    let mut frame = Frame::new(body, proto_id);
    frame.header.serial_no = serial_no;
    w.write_all(&frame.to_bytes().unwrap()).await.unwrap();
}

async fn next_request(r: &mut FrameReader<OwnedReadHalf>) -> FrameRaw {
    loop {
        let frame = r.read_frame_raw().await.unwrap().unwrap();
        if frame.header.proto_id != KEEPALIVE {
            return frame;
        }
    }
}

fn sub_resp(ret_type: i32) -> futuapi_rs::Qot_Sub::Response {
    let mut resp = futuapi_rs::Qot_Sub::Response::new();
    resp.set_retType(ret_type);
    resp.set_retMsg("boom".into());
    resp
}

fn basic_qot_push() -> futuapi_rs::Qot_UpdateBasicQot::Response {
    let mut resp = futuapi_rs::Qot_UpdateBasicQot::Response::new();
    resp.set_retType(0);
    resp.s2c = MessageField::some(Default::default());
    resp
}

async fn mock_opend(listener: TcpListener) {
    let (socket, _) = listener.accept().await.unwrap();
    let (r, mut w) = socket.into_split();
    let mut r = FrameReader::new(r);

    let req = next_request(&mut r).await;
    assert_eq!(req.header.proto_id, INIT_CONNECT);
    let mut s2c = futuapi_rs::InitConnect::S2C::new();
    s2c.set_serverVer(1);
    s2c.set_loginUserID(1);
    s2c.set_connID(1);
    s2c.set_connAESKey("0123456789abcdef".into());
    s2c.set_keepAliveInterval(1);
    let mut resp = futuapi_rs::InitConnect::Response::new();
    resp.set_retType(0);
    resp.s2c = MessageField::some(s2c);
    send(&mut w, resp, INIT_CONNECT, req.header.serial_no).await;

    // a keepalive reply and a push land before the (failed) Qot_Sub response
    let req = next_request(&mut r).await;
    assert_eq!(req.header.proto_id, SUB);
    let mut keepalive = futuapi_rs::KeepAlive::Response::new();
    keepalive.set_retType(0);
    keepalive.s2c.mut_or_insert_default().set_time(0);
    send(&mut w, keepalive, KEEPALIVE, 0).await;
    send(&mut w, basic_qot_push(), UPDATE_BASIC_QOT, 0).await;
    send(&mut w, sub_resp(-1), SUB, req.header.serial_no).await;

    let req = next_request(&mut r).await;
    assert_eq!(req.header.proto_id, SUB);
    send(&mut w, sub_resp(0), SUB, req.header.serial_no).await;

    // the client is blocked in next_data now; its keepalive must still get through
    let req = timeout(Duration::from_secs(3), r.read_frame_raw())
        .await
        .expect("no keepalive while the subscriber was waiting")
        .unwrap()
        .unwrap();
    assert_eq!(req.header.proto_id, KEEPALIVE);
    send(&mut w, basic_qot_push(), UPDATE_BASIC_QOT, 0).await;
}

#[tokio::test]
async fn routes_responses_and_pushes() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(mock_opend(listener));

    let (sub_client, mut subscriber) = client::sub_connect(addr).await.unwrap();

    let err = sub_client.subscribe(sub_req()).await.unwrap_err();
    assert!(err.to_string().contains("boom"), "{}", err);
    sub_client.subscribe(sub_req()).await.unwrap();

    // the push that arrived ahead of the Qot_Sub response; the keepalive reply is skipped
    assert!(matches!(
        subscriber.next_data().await.unwrap(),
        Some(UpdateResponse::BasicQot(_))
    ));
    assert!(matches!(
        subscriber.next_data().await.unwrap(),
        Some(UpdateResponse::BasicQot(_))
    ));

    server.await.unwrap();
    assert!(subscriber.next_data().await.unwrap().is_none());

    let resp = timeout(Duration::from_secs(1), sub_client.subscribe(sub_req()))
        .await
        .expect("request on a closed connection hung");
    assert!(resp.is_err());
}
