use atomic_counter::{AtomicCounter, ConsistentCounter};
use bytes::Buf;
use crypto::{digest::Digest, sha1::Sha1};
use protobuf::MessageFull;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

const PROTO_HEADER_LEN: usize = 44;
const HEADER_FLAG: [u8; 2] = [70, 84]; // "FT"

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref SERIAL_NO: ConsistentCounter = ConsistentCounter::new(0);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIProtoHeader {
    header_flag: [u8; 2],
    proto_id: u32,
    proto_fmt_type: u8,
    proto_ver: u8,
    serial_no: u32,
    body_len: u32,
    body_sha1: [u8; 20],
    reserved: [u8; 8],
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
    Other(String),
}

#[derive(Debug)]
pub struct Frame<T: MessageFull> {
    pub header: APIProtoHeader,
    pub body: T,
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

    let decoded = bincode::deserialize(src.get_ref().get(0..PROTO_HEADER_LEN).unwrap()).unwrap();

    src.advance(PROTO_HEADER_LEN);

    Ok(decoded)
}

fn get_body<T: MessageFull>(src: &mut Cursor<&[u8]>, len: u32) -> Result<T, Error> {
    let remaining = src.remaining() as u32;
    if remaining < len {
        return Err(Error::Incomplete);
    }

    Ok(T::parse_from_bytes(src.copy_to_bytes(len as usize).as_ref()).unwrap())
}

pub fn serial_no() -> u32 {
    SERIAL_NO.inc();
    SERIAL_NO.get() as u32
}

impl<T: MessageFull> Frame<T> {
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
            return Err(Error::Other(
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
