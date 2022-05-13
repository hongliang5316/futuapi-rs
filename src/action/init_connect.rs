use crate::frame::Frame;
use crate::Common::RetType;
use crate::InitConnect::{Request, Response, C2S};
use protobuf::MessageField;

const PROTO_ID: u32 = 1001;

#[derive(Debug)]
pub struct InitConnectRequest {
    client_ver: i32,
    client_id: String,
    recv_notify: bool,
    package_enc_algo: i32,
    programming_language: String,
}

impl Into<Request> for InitConnectRequest {
    fn into(self) -> Request {
        let mut req = Request::new();
        let mut c2s = C2S::new();
        c2s.set_clientVer(self.client_ver);
        c2s.set_clientID(self.client_id);
        c2s.set_recvNotify(self.recv_notify);
        c2s.set_packetEncAlgo(self.package_enc_algo);
        c2s.set_programmingLanguage(self.programming_language);
        req.c2s = MessageField::some(c2s);

        req
    }
}

impl InitConnectRequest {
    pub fn into_frame(self) -> Frame<Request> {
        Frame::new(self.into(), PROTO_ID)
    }
}

impl Default for InitConnectRequest {
    fn default() -> Self {
        InitConnectRequest {
            client_ver: 221,
            client_id: "800".into(),
            recv_notify: false,
            package_enc_algo: -1,
            programming_language: "Rust".into(),
        }
    }
}

#[derive(Debug)]
pub struct InitConnectResponse {
    server_ver: i32,
    login_user_id: u64,
    conn_id: u64,
    conn_aes_key: String,
    keep_alive_interval: i32,
    aes_cbc_iv: String,
    user_attribution: i32,
}

impl From<Response> for InitConnectResponse {
    fn from(resp: Response) -> Self {
        InitConnectResponse {
            server_ver: resp.s2c.serverVer(),
            login_user_id: resp.s2c.loginUserID(),
            conn_id: resp.s2c.connID(),
            conn_aes_key: resp.s2c.connAESKey().into(),
            keep_alive_interval: resp.s2c.keepAliveInterval(),
            aes_cbc_iv: resp.s2c.aesCBCiv().into(),
            user_attribution: resp.s2c.userAttribution(),
        }
    }
}

pub fn check_response(resp: Response) -> crate::Result<InitConnectResponse> {
    if resp.retType() == RetType::RetType_Succeed as i32 {
        return Ok(InitConnectResponse::from(resp));
    }

    Err(resp.retMsg().into())
}
