use bytes::Buf;
use crypto::{digest::Digest, sha1::Sha1};
use protobuf::MessageFull;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::Cursor,
    sync::{Arc, Mutex},
};

const PROTO_HEADER_LEN: usize = 44;
const HEADER_FLAG: [u8; 2] = [70, 84]; // "FT"

lazy_static! {
    static ref SERIAL_NO: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIProtoHeader {
    pub header_flag: [u8; 2],
    pub proto_id: u32,
    pub proto_fmt_type: u8,
    pub proto_ver: u8,
    pub serial_no: u32,
    pub body_len: u32,
    pub body_sha1: [u8; 20],
    pub reserved: [u8; 8],
}

impl APIProtoHeader {
    pub fn to_vec(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

#[derive(Debug)]
pub enum Error {
    Incomplete,
    ProtoError(String),
    ConnectionError(String),
    Timeout(String),
    Other(String),
}

#[derive(Debug)]
pub struct Frame<T: MessageFull> {
    pub header: APIProtoHeader,
    pub body: T,
}

#[derive(Debug)]
pub struct FrameRaw {
    pub header: APIProtoHeader,
    pub body: Vec<u8>,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

fn sha1(msg: &[u8]) -> [u8; 20] {
    let mut buf: [u8; 20] = Default::default();
    let mut hasher = Sha1::new();
    hasher.input(msg);
    hasher.result(&mut buf);
    buf
}

fn get_header(src: &mut Cursor<&[u8]>) -> Result<APIProtoHeader, Error> {
    let remaining = src.remaining();
    if remaining < PROTO_HEADER_LEN {
        return Err(Error::Incomplete);
    }

    let decoded = match bincode::deserialize(src.get_ref().get(0..PROTO_HEADER_LEN).unwrap()) {
        Ok(d) => d,
        Err(e) => return Err(Error::ProtoError(format!("failed to decode header: {}", e))),
    };

    src.advance(PROTO_HEADER_LEN);

    Ok(decoded)
}

fn get_body<T: MessageFull>(src: &mut Cursor<&[u8]>, len: u32) -> Result<T, Error> {
    let remaining = src.remaining() as u32;
    if remaining < len {
        return Err(Error::Incomplete);
    }

    T::parse_from_bytes(src.copy_to_bytes(len as usize).as_ref())
        .map_err(|e| Error::ProtoError(format!("failed to decode body: {}", e)))
}

fn get_body_raw(src: &mut Cursor<&[u8]>, len: u32) -> Result<Vec<u8>, Error> {
    let remaining = src.remaining() as u32;
    if remaining < len {
        return Err(Error::Incomplete);
    }

    Ok(src.copy_to_bytes(len as usize).to_vec())
}

pub fn serial_no() -> u32 {
    let mut n = SERIAL_NO.lock().unwrap();
    *n += 1;
    *n
}

impl FrameRaw {
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<FrameRaw, Error> {
        let header = get_header(src)?;
        let body = get_body_raw(src, header.body_len)?;

        Ok(FrameRaw { header, body })
    }
}

impl<T: MessageFull> Frame<T> {
    pub fn from_raw(frame_raw: FrameRaw) -> Result<Frame<T>, Error> {
        Ok(Frame {
            header: frame_raw.header,
            body: T::parse_from_bytes(&frame_raw.body)
                .map_err(|e| Error::ProtoError(format!("failed to decode body: {}", e)))?,
        })
    }

    pub fn new(body: T, proto_id: u32) -> Frame<T> {
        let b = body.write_to_bytes().unwrap();

        Frame {
            header: APIProtoHeader {
                header_flag: HEADER_FLAG,
                proto_id,
                proto_fmt_type: 0, // 0: protobuf 1: json
                proto_ver: 0,
                serial_no: serial_no(),
                body_len: b.len() as u32,
                body_sha1: sha1(&b),
                reserved: Default::default(),
            },
            body,
        }
    }

    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame<T>, Error> {
        let header = get_header(src)?;
        if header.proto_fmt_type != 0 {
            return Err(Error::ProtoError(
                "Unsupported protocol format type: json".into(),
            ));
        }

        let body_len = header.body_len;
        Ok(Frame {
            header,
            body: get_body(src, body_len)?,
        })
    }
}
